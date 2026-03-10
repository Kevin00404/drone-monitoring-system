use super::posicion::Posicion;

/// Trayectoria rectilinea para que un dron la circule
#[derive(Debug, Clone)]
pub struct Recta {
    a: f64,
    b: f64,
    origen: Posicion,
    destino: Posicion,
    t: f64,
    incremento: f64,
}

impl Recta {
    pub fn new(origen: Posicion, destino: Posicion, incremento: f64) -> Recta {
        Recta {
            a: destino.lat - origen.lat,
            b: destino.long - origen.long,
            origen,
            destino,
            t: 0.0,
            incremento,
        }
    }

    //t entre 0 y 1.
    pub fn mover(&mut self) -> (f64, f64) {
        self.t = if self.t + self.incremento > 1.0 {
            1.0
        } else {
            self.t + self.incremento
        };

        if self.t == 1.0 {
            return (self.destino.lat, self.destino.long);
        }

        let x = self.t.mul_add(self.a, self.origen.lat);
        let y = self.t.mul_add(self.b, self.origen.long);

        (x, y)
    }

    pub fn get_destino(&self) -> Posicion {
        self.destino.clone()
    }
    pub fn get_origen(&self) -> Posicion {
        self.origen.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::dron::posicion::Posicion;

    use super::Recta;

    #[test]
    fn recta_arroja_valores_correctos_al_usar_mover() {
        let origen = Posicion::new(2.0, 4.0);
        let destino = Posicion::new(2.3, 3.5);

        let mut r1 = Recta::new(origen, destino, 0.005 * 30.0);

        for _ in 1..7 {
            println!("Avanzo a {:#?}", r1.mover());
        }

        let origen = Posicion::new(2.0, 4.0);
        let destino = Posicion::new(2.3, 3.5);

        let mut r2 = Recta::new(destino, origen, 0.005 * 30.0);

        for _ in 1..7 {
            println!("Avanzo a {:#?}", r2.mover());
        }

        assert_eq!((2.3, 3.5), r1.mover());
        assert_eq!((2.0, 4.0), r2.mover());
    }

    #[test]
    fn una_vez_llegado_al_final_de_la_trayectoria_da_todo_el_tiempo_la_pos_final() {
        let origen = Posicion::new(2.0, 4.0);
        let destino = Posicion::new(2.3, 3.5);

        let mut r1 = Recta::new(origen, destino, 0.005 * 30.0);

        for _ in 1..7 {
            println!("Avanzo a {:#?}", r1.mover());
        }

        assert_eq!((2.3, 3.5), r1.mover());
        assert_eq!((2.3, 3.5), r1.mover());
        assert_eq!((2.3, 3.5), r1.mover());
        assert_eq!((2.3, 3.5), r1.mover());
        assert_eq!((2.3, 3.5), r1.mover());
        assert_eq!((2.3, 3.5), r1.mover());
        assert_eq!((2.3, 3.5), r1.mover());
    }
}
