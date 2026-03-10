use std::fmt;

use super::direccion::Direccion;

#[derive(Debug, Clone, PartialEq)]
/// Representa los posibles estados en los que puede encontrarse un dron
pub enum EstadoDron {
    Patrullando,
    Encamino(Direccion),
    Resolviendo,
    Recargando,
}

// Implementación del trait Display para EstadoCamara
impl fmt::Display for EstadoDron {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EstadoDron::Patrullando => write!(f, "patrullando"),
            EstadoDron::Encamino(dir) => write!(f, "enCamino{}", dir),
            EstadoDron::Resolviendo => write!(f, "resolviendo"),
            EstadoDron::Recargando => write!(f, "recargando"),
        }
    }
}
