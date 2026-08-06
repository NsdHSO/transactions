use crate::factory_method::type_cargo::CargoType;

#[derive(Debug)]
pub struct Ship {
    pub fuel: bool,
    pub cargo: CargoType,
}
