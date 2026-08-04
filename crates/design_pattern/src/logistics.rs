#[derive(Debug)]
pub struct Truck {
    pub whells: u32,
}

#[derive(Debug)]
pub struct Logistics {
    pub trucks: Vec<Truck>,
}
