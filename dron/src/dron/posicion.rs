/// Posicion: Representa una Posicionenada en el plano 2d
#[derive(Debug, Clone, PartialEq)]
pub struct Posicion {
    pub lat: f64,
    pub long: f64,
}

impl Posicion {
    pub const fn new(lat: f64, long: f64) -> Self {
        Posicion { lat, long }
    }

    /// Devuelve true o false en base si el dron se encuentra dentro del area dada
    pub fn en_rango(&self, lat: f64, long: f64, area: f64) -> bool {
        println!(
            "Comparando posiciones: {}, {}, {}, {}, {}",
            self.lat, self.long, lat, long, area
        );
        println!("(self.lat - lat).powi(2): {}", (self.lat - lat).powi(2));
        println!("(self.long - long).powi(2): {}", (self.long - long).powi(2));
        (self.lat - lat).hypot(self.long - long) <= area
    }

    pub fn match_pos(&self, pos: &Posicion) -> bool {
        self.lat == pos.lat && self.long == pos.long
    }
}
