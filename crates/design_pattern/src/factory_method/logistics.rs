use crate::factory_method::ship::Ship;

#[derive(Debug, Clone)]
pub struct Truck {
    pub whells: u32,
}

#[derive(Debug, Clone)]
pub struct Logistics {
    pub trucks: Vec<Truck>,
    pub ships: Vec<Ship>,
}

impl Logistics {
    pub fn count_how_many_was_dispaced(&self) {
        println!(
            "Trucks: {:?}\n Ships: {:?}",
            self.trucks.len(),
            self.ships.len()
        )
    }
}
