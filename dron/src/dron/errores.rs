#[derive(Debug)]
pub enum ErroresDron {
    ErrorAlCrearDron,
    InternalError(String),
}

impl std::fmt::Display for ErroresDron {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErroresDron::ErrorAlCrearDron => write!(f, "Hubo un error al crear el dron"),
            ErroresDron::InternalError(msg) => write!(f, "Error interno: {}", msg),
        }
    }
}
impl From<ErroresDron> for std::io::Error {
    fn from(err: ErroresDron) -> Self {
        match err {
            ErroresDron::ErrorAlCrearDron => {
                std::io::Error::new(std::io::ErrorKind::Other, "Hubo un error al crear el dron")
            }
            ErroresDron::InternalError(msg) => {
                std::io::Error::new(std::io::ErrorKind::Other, format!("Error interno: {}", msg))
            }
        }
    }
}
