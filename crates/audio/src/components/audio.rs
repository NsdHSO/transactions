use coreaudio_sys::*;
use std::ffi::CStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{iter, ptr};

#[expect(dead_code)]
const K_AUDIO_HARDWARE_PROPERTY_RUNNING_PROCESS_LIST: u32 = 0x726e706c;

#[expect(dead_code)]
const K_AUDIO_HARDWARE_PROPERTY_PROCESS_VOLUME: u32 = 0x70726376;

const K_AUDIO_HARDWARE_PROPERTY_PROCESS_VOLUME_1: u32 = 0x70766f6c;

#[repr(C)]
#[expect(dead_code)]
struct AudioHardwareProcessList {
    m_pid: i32,
    m_running: u32,
}
#[derive(Default, Clone, Debug)]
pub struct AudioProcessInfo {
    pub pid: i32,
    pub name: String,
    pub is_running: bool,
    pub volume: f32, //0.0 - 1.0
}

#[derive(Default, Clone, Debug)]
pub struct AudioCapture {
    pub processes: Arc<Mutex<Vec<AudioProcessInfo>>>,
    running: Arc<AtomicBool>,
}

impl AudioCapture {
    pub fn new() -> Self {
        let processes = Arc::new(Mutex::new(Vec::new()));
        let running = Arc::new(AtomicBool::new(true));

        let p = processes.clone();
        let r = running.clone();

        std::thread::spawn(move || {
            let poll = iter::from_fn(move || {
                if !r.load(Ordering::Relaxed) {
                    return None;
                };
                std::thread::sleep(std::time::Duration::from_secs(2));
                Some(enumerate_running_processes())
            });

            poll.for_each(|infos| {
                if let Ok(mut guard) = p.lock() {
                    *guard = infos;
                }
            })
        });

        Self { processes, running }
    }
}

impl Drop for AudioCapture {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

fn pid_to_name(pid: i32) -> String {
    let mut buf = [0i8; 4096];
    let len = unsafe {
        libc::proc_name(
            pid,
            buf.as_mut_ptr() as *mut std::ffi::c_void,
            buf.len() as u32,
        )
    };
    if len > 0 {
        unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned() }
    } else {
        format!("PID {}", pid)
    }
}

fn enumerate_running_processes() -> Vec<AudioProcessInfo> {
    let addr = AudioObjectPropertyAddress {
        mSelector: kAudioHardwarePropertyProcessObjectList,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMain,
    };
    let mut data_size: u32 = 0;
    let status = unsafe {
        AudioObjectGetPropertyDataSize(
            kAudioObjectSystemObject,
            &addr as *const AudioObjectPropertyAddress,
            0,
            ptr::null(),
            &mut data_size as *mut u32,
        )
    };
    if status != 0 || data_size == 0 {
        return vec![];
    }
    let count = (data_size as usize) / std::mem::size_of::<AudioObjectID>();
    let mut objects: Vec<AudioObjectID> = vec![0; count];
    let status = unsafe {
        AudioObjectGetPropertyData(
            kAudioObjectSystemObject,
            &addr as *const AudioObjectPropertyAddress,
            0,
            ptr::null(),
            &mut data_size as *mut u32,
            objects.as_mut_ptr() as *mut std::ffi::c_void,
        )
    };
    if status != 0 {
        return vec![];
    }
    let mut results = Vec::new();

    for &obj_id in &objects {
        let pid_addr = AudioObjectPropertyAddress {
            mSelector: kAudioProcessPropertyPID,
            mScope: kAudioObjectPropertyScopeGlobal,
            mElement: kAudioObjectPropertyElementMain,
        };

        let mut pid: i32 = 0;
        let mut pid_size = std::mem::size_of::<i32>() as u32;
        let pid_status = unsafe {
            AudioObjectGetPropertyData(
                obj_id,
                &pid_addr as *const AudioObjectPropertyAddress,
                0,
                ptr::null(),
                &mut pid_size as *mut u32,
                &mut pid as *mut i32 as *mut std::ffi::c_void,
            )
        };

        if pid_status != 0 {
            continue;
        }
        try_get_volume_prcv_sys(pid);
        try_get_volume_pvol_obj(obj_id);
        try_get_volume_prcv_obj(obj_id);
        let running_addr = AudioObjectPropertyAddress {
            mSelector: kAudioProcessPropertyIsRunningOutput,
            mScope: kAudioObjectPropertyScopeGlobal,
            mElement: kAudioObjectPropertyElementMain,
        };
        let mut running: u32 = 0;
        let mut running_size = std::mem::size_of::<u32>() as u32;
        let running_status = unsafe {
            AudioObjectGetPropertyData(
                obj_id,
                &running_addr as *const AudioObjectPropertyAddress,
                0,
                ptr::null(),
                &mut running_size as *mut u32,
                &mut running as *mut u32 as *mut std::ffi::c_void,
            )
        };
        let is_running = running_status == 0 && running != 0;
        let name = pid_to_name(pid);
        let volume = 0.0;
        results.push(AudioProcessInfo {
            pid,
            name,
            is_running,
            volume,
        });
    }
    results
}

fn try_get_volume_prcv_sys(pid: i32) -> Option<f32> {
    let addr = AudioObjectPropertyAddress {
        mSelector: 0x70726376, // 'prcv'
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: pid as u32,
    };
    let mut volume: f32 = 0.0;
    let mut size = std::mem::size_of::<f32>() as u32;
    let status = unsafe {
        AudioObjectGetPropertyData(
            kAudioObjectSystemObject,
            &addr as *const AudioObjectPropertyAddress,
            0,
            ptr::null(),
            &mut size as *mut u32,
            &mut volume as *mut f32 as *mut std::ffi::c_void,
        )
    };
    eprintln!("prcv sys pid={pid} status={status} vol={volume}");
    if status == 0 { Some(volume) } else { None }
}

fn try_get_volume_pvol_obj(obj_id: AudioObjectID) -> Option<f32> {
    let addr = AudioObjectPropertyAddress {
        mSelector: 0x70766f6c, // 'pvol'
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMain,
    };
    let mut volume: f32 = 0.0;
    let mut size = std::mem::size_of::<f32>() as u32;
    let status = unsafe {
        AudioObjectGetPropertyData(
            obj_id,
            &addr as *const AudioObjectPropertyAddress,
            0,
            ptr::null(),
            &mut size as *mut u32,
            &mut volume as *mut f32 as *mut std::ffi::c_void,
        )
    };
    eprintln!("pvol obj={obj_id} status={status} vol={volume}");
    if status == 0 { Some(volume) } else { None }
}

fn try_get_volume_prcv_obj(obj_id: AudioObjectID) -> Option<f32> {
    let addr = AudioObjectPropertyAddress {
        mSelector: 0x70726376, // 'prcv'
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMain,
    };
    let mut volume: f32 = 0.0;
    let mut size = std::mem::size_of::<f32>() as u32;
    let status = unsafe {
        AudioObjectGetPropertyData(
            obj_id,
            &addr as *const AudioObjectPropertyAddress,
            0,
            ptr::null(),
            &mut size as *mut u32,
            &mut volume as *mut f32 as *mut std::ffi::c_void,
        )
    };
    eprintln!("prcv obj={obj_id} status={status} vol={volume}");
    if status == 0 { Some(volume) } else { None }
}
