use super::{circunferencia::Circunferencia, recta::Recta};
#[derive(Debug, Clone)]
/// Representa las posibles trayectorias que puede seguir un dron
pub enum Trayectoria {
    Recta(Recta),
    Circunferencia(Circunferencia),
}
