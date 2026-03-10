use std::fmt;

use super::posicion::Posicion;

#[derive(Debug, Clone, PartialEq)]
/// Representa la direccion hacia la que se dirige un dron
pub enum Direccion {
    Incidente(Posicion),
    AreaDeOperaciones,
    Base,
}

impl fmt::Display for Direccion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direccion::Incidente(_) => write!(f, "Incidente"),
            Direccion::AreaDeOperaciones => write!(f, "AreaOp"),
            Direccion::Base => write!(f, "Base"),
        }
    }
}
