use coreaudio_sys::*;
use std::iter;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[expect(dead_code)]
const K_AUDIO_HARDWARE_PROPERTY_RUNNING_PROCESS_LIST: u32 = 0x726e706c;

#[expect(dead_code)]
const K_AUDIO_HARDWARE_PROPERTY_PROCESS_VOLUME: u32 = 0x70726376;

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
                if r.load(Ordering::Relaxed) {
                    return None;
                };
                let pids = enumerate_running_processes();
                let infos: Vec<AudioProcessInfo> = pids
                    .iter()
                    .map(|(pid, is_running)| AudioProcessInfo {
                        pid: *pid,
                        is_running: *is_running,
                        name: pid_to_name(*pid),
                        volume: get_process_volume(*pid),
                    })
                    .collect();

                std::thread::sleep(std::time::Duration::from_secs(2));
                Some(infos)
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

fn get_process_volume(p0: i32) -> f32 {
    2.3
}

fn pid_to_name(p0: i32) -> String {
    String::from("Iancu")
}

fn enumerate_running_processes() -> Vec<(i32, bool)> {
    vec![(2, true), (3, false)]
}
