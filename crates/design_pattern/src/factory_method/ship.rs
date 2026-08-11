use crate::factory_method::type_cargo::CargoType;

#[derive(Debug, Clone)]
pub struct Ship {
    pub fuel: bool,
    pub cargo: CargoType,
}
