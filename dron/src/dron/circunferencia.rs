use super::posicion::Posicion;
use std::f64::consts::PI;

/// Trayectoria Circular que utiliza el dron para moverse 
#[derive(Debug, Clone)]
pub struct Circunferencia {
    angulo: f64,
    base_central: Posicion,
    radio: f64,
    incremento: f64,
}

impl Circunferencia {
    pub fn new(
        mut angulo: f64,
        base_central: Posicion,
        radio: f64,
        incremento: f64,
    ) -> Circunferencia {
        if 2.0f64.mul_add(-PI, angulo).abs() < 0.0001 {
            angulo = 0.0;
        }

        Circunferencia {
            angulo,
            base_central,
            radio,
            incremento,
        }
    }

    /// El dron se mueve
    pub fn mover(&mut self) -> (f64, f64) {
        //El movimiento es contra reloj.
        let operacion = PI.mul_add(self.incremento, self.angulo);

        self.angulo = if operacion == 2.0 * PI {
            0.0
        } else {
            operacion
        };

        (
            self.radio.mul_add(f64::cos(self.angulo), self.base_central.lat),
            self.radio.mul_add(f64::sin(self.angulo), self.base_central.long),
        )
    }

    pub const fn get_angulo(&self) -> &f64 {
        &self.angulo
    }
}

/// devuelve el punto mas cercano de la circunferencia al dron
pub fn pto_mas_cercano_circunferencia(
    h: f64,  // coordenada x del centro de la circunferencia
    k: f64,  // coordenada y del centro de la circunferencia
    r: f64,  // radio de la circunferencia
    x1: f64, // coordenada x del punto dado
    y1: f64, // coordenada y del punto dado
) -> (f64, f64) {
    let presicion = 0.00001;
    if ((x1 - h).abs() < presicion) && ((y1 - k).abs() < presicion) {
        return (h + r, k);
    }

    let dx = x1 - h;
    let dy = y1 - k;

    let dist = dx.hypot(dy);
    println!("Dist: {:#?}", dist);

    let x_cercano = if dx != 0.0 || dy != 0.0 {
        r.mul_add(dx / dist, h)
    } else {
        0.0
    };

    let y_cercano = if dy != 0.0 || dx != 0.0 {
        r.mul_add(dy / dist, k)
    } else {
        0.0
    };

    (x_cercano, y_cercano)
}

#[cfg(test)]
mod test {
    use crate::dron::{circunferencia::pto_mas_cercano_circunferencia, posicion::Posicion};

    use super::Circunferencia;

    #[test]
    fn pto_mas_cercano_circunferencia_ok() {
        let centro = (1.0, 2.0);
        let radio = 2.0;
        let punto1 = (1.0, -1.0);
        let punto2 = (4.0, 2.0);
        let punto3 = (1.0, 5.0);
        let punto4 = (-2.0, 2.0);
        let punto5 = (3.0, 4.0);
        let punto6 = (0.0, 0.0);
        let punto7 = (-1.0, 0.0);

        assert_eq!(
            (1.0, 0.0),
            pto_mas_cercano_circunferencia(centro.0, centro.1, radio, punto1.0, punto1.1)
        );
        assert_eq!(
            (3.0, 2.0),
            pto_mas_cercano_circunferencia(centro.0, centro.1, radio, punto2.0, punto2.1)
        );
        assert_eq!(
            (1.0, 4.0),
            pto_mas_cercano_circunferencia(centro.0, centro.1, radio, punto3.0, punto3.1)
        );
        assert_eq!(
            (-1.0, 2.0),
            pto_mas_cercano_circunferencia(centro.0, centro.1, radio, punto4.0, punto4.1)
        );
        assert_eq!(
            (2.414213562373095, 3.414213562373095),
            pto_mas_cercano_circunferencia(centro.0, centro.1, radio, punto5.0, punto5.1)
        );
        assert_eq!(
            (0.10557280900008414, 0.2111456180001683),
            pto_mas_cercano_circunferencia(centro.0, centro.1, radio, punto6.0, punto6.1)
        );
        assert_eq!(
            (-0.4142135623730949, 0.5857864376269051),
            pto_mas_cercano_circunferencia(centro.0, centro.1, radio, punto7.0, punto7.1)
        );
    }

    #[test]
    fn me_muevo_correctamente_al_rededor_de_la_circunferencia() {
        let base_central = Posicion::new(0.0, 0.0);
        let radio = 2.0;
        let incremento = 0.5;
        let angulo = 0.0;

        let mut c = Circunferencia::new(angulo, base_central, radio, incremento);

        let p1 = c.mover();
        assert_eq!((0.0, 2.0), (p1.0.round(), p1.1));
        let p2 = c.mover();
        assert_eq!((-2.0, 0.0), (p2.0, p2.1.round()));
        let p3 = c.mover();
        assert_eq!((0.0, -2.0), (p3.0.round(), p3.1));
        let p4 = c.mover();
        assert_eq!((2.0, 0.0), (p4.0, p4.1.round()));
    }

    #[test]
    fn me_muevo_correctamente_al_rededor_de_la_circunferencia_no_centrada_en_el_origen() {
        let base_central = Posicion::new(1.0, 2.0);
        let radio = 2.0;
        let incremento = 0.5;
        let angulo = 0.0;

        let mut c = Circunferencia::new(angulo, base_central, radio, incremento);

        let p1 = c.mover();
        assert_eq!((1.0, 4.0), (p1.0.round(), p1.1));
        let p2 = c.mover();
        assert_eq!((-1.0, 2.0), (p2.0, p2.1.round()));
        let p3 = c.mover();
        assert_eq!((1.0, 0.0), (p3.0.round(), p3.1));
        let p4 = c.mover();
        assert_eq!((3.0, 2.0), (p4.0, p4.1.round()));
    }
}
