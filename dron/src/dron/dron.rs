use std::f32::consts::PI;

use crate::dron::posicion::Posicion;

use super::circunferencia::{self, Circunferencia};
use super::direccion::Direccion;
use super::estadodron::EstadoDron;
use super::recta::Recta;
use super::trayectoria::Trayectoria;

/// Dron se crea en estado en camino al area de op y puede ir a resolver un incidente en su area  
#[derive(Debug, Clone)]
pub struct Dron {
    id: i32,
    estado: EstadoDron,
    trayectoria: Trayectoria,
    posicion: Posicion,
    bateria: f64,
    min_bat: f64,
    area_incidente: f64,
    radio: f64,
    velocidad: f64,
    base_central: Posicion,
    consumo_por_movimiento: f64,
    unidad_de_movimiento: f64,
}

impl Dron {
    /// Crea un nuevo dron
    pub fn new(
        id: i32,
        longitud: f64,
        latitud: f64,
        radio: f64,
        velocidad: f64,
        bateria: f64,
        area_incidente: f64,
    ) -> Self {
        let pos = Posicion {
            lat: latitud,
            long: longitud,
        };

        let destino = Posicion {
            lat: latitud + radio,
            long: longitud,
        };

        let consumo_por_movimiento = 0.01;
        let unidad_de_movimiento = 0.001;

        Dron {
            id,
            estado: EstadoDron::Encamino(Direccion::AreaDeOperaciones),
            trayectoria: Trayectoria::Recta(Recta::new(
                pos.clone(),
                destino,
                unidad_de_movimiento * velocidad,
            )),
            posicion: pos,
            bateria,
            min_bat: 25.0,
            radio,
            velocidad,
            area_incidente,
            base_central: Posicion {
                lat: latitud,
                long: longitud,
            },
            consumo_por_movimiento,
            unidad_de_movimiento,
        }
    }

    /// Evalua si el dron esta disponible para atender un incidente
    pub fn disponible(&self, latitud: f64, longitud: f64) -> bool {
        println!("Poca bateria: {}", self.poca_bateria());
        println!("Ocupado: {}", self.ocupado());
        println!(
            "Disponible: {}",
            self.en_rango(latitud, longitud) && !self.poca_bateria() && !self.ocupado()
        );
        self.en_rango(latitud, longitud) && !self.poca_bateria() && !self.ocupado()
    }

    /// Evalua si el dron esta en rango de un incidente
    pub fn en_rango(&self, latitud: f64, longitud: f64) -> bool {
        println!(
            "En rango: {}",
            self.posicion
                .en_rango(latitud, longitud, self.area_incidente)
        );
        self.posicion
            .en_rango(latitud, longitud, self.area_incidente)
    }

    /// Evalua si el dron tiene un id en su lista de ids
    pub fn match_id(&self, vec: Vec<i32>) -> bool {
        vec.contains(&self.id)
    }

    /// Evalua si el dron esta ocupado
    pub fn ocupado(&self) -> bool {
        match &self.estado {
            EstadoDron::Encamino(Direccion::AreaDeOperaciones) => match &self.trayectoria {
                Trayectoria::Recta(r) => {
                    let origen = r.get_origen();
                    let presicion = 0.000001;

                    !(origen.lat - self.base_central.lat < presicion && origen.long - self.base_central.long < presicion)
                }
                _ => true,
            },
            EstadoDron::Encamino(_) => true,
            EstadoDron::Patrullando => false,
            EstadoDron::Resolviendo => true,
            EstadoDron::Recargando => !self.mas_del_doble_que_el_min(),
        }
    }

    /// Evalua si la bateria del dron es mayor al doble del minimo
    pub fn mas_del_doble_que_el_min(&self) -> bool {
        self.bateria >= 2.0 * self.min_bat
    }

    /// Evalua si la bateria del dron esta cargada
    pub fn cargado(&self) -> bool {
        self.bateria == 100.0
    }

    /// Mueve el dron considerando su trayectoria
    pub fn mover(&mut self) -> Posicion {
        match &mut self.trayectoria {
            Trayectoria::Circunferencia(c) => {
                let pos = c.mover();
                self.posicion = Posicion::new(pos.0, pos.1);
            }
            Trayectoria::Recta(r) => {
                let pos = r.mover();
                self.posicion = Posicion::new(pos.0, pos.1);
            }
        }
        self.posicion.clone()
    }

    /// Evalua si el dron tiene poca bateria
    pub fn poca_bateria(&self) -> bool {
        self.bateria < self.min_bat
    }

    /// Evalua si el dron llego a su destino
    pub fn llegaste(&self) -> bool {
        match &self.estado {
            EstadoDron::Encamino(Direccion::AreaDeOperaciones) => match &self.trayectoria {
                Trayectoria::Recta(r) => self.posicion.match_pos(&r.get_destino()),
                _ => true,
            },
            EstadoDron::Encamino(Direccion::Incidente(pos)) => self.posicion.match_pos(pos),
            EstadoDron::Encamino(Direccion::Base) => self.posicion.match_pos(&self.base_central),
            _ => true,
        }
    }

    pub fn set_estado_patrullando(&mut self) {
        self.estado = EstadoDron::Patrullando;
        let angulo = f64::asin((self.posicion.long - self.base_central.long).abs() / self.radio); //El angulo de ese punto de la circunferencia
        self.trayectoria = Trayectoria::Circunferencia(Circunferencia::new(
            self.detectar_cuadrante(angulo),
            self.base_central.clone(),
            self.radio,
            self.unidad_de_movimiento * self.velocidad,
        ));
    }

    /// Detecta en que cuadrante se encuentra el dron
    fn detectar_cuadrante(&self, angulo: f64) -> f64 {
        let x = self.posicion.lat;
        let y = self.posicion.long;

        let devolver;
        if x < self.base_central.lat && y >= self.base_central.long {
            devolver = PI as f64 - angulo
        } else if x <= self.base_central.lat && y < self.base_central.long {
            devolver = angulo.abs() + PI as f64;
        } else if x > self.base_central.lat && y <= self.base_central.long {
            devolver = 2.0f64.mul_add(PI as f64, -angulo.abs());
        } else {
            devolver = angulo;
        }

        println!("El angulo {}", devolver);
        devolver
    }

    pub fn set_estado_recargando(&mut self) {
        self.estado = EstadoDron::Recargando;
        self.posicion.lat = self.base_central.lat;
        self.posicion.long = self.base_central.long;
    }

    pub fn cargar_bateria(&mut self) {
        let aux = 2.0f64.mul_add(self.consumo_por_movimiento, self.bateria);
        if aux > 100.0 {
            self.bateria = 100.0;
        } else {
            self.bateria = aux;
        }
    }

    pub fn gastar_bateria(&mut self) {
        self.bateria -= self.consumo_por_movimiento;
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    pub fn set_longitud(&mut self, longitud: f64) {
        self.posicion.long = longitud;
    }

    pub fn set_radio(&mut self, radio: f64) {
        self.radio = radio;
    }

    pub fn set_latitud(&mut self, latitud: f64) {
        self.posicion.lat = latitud;
    }

    pub fn set_estado_en_camino(&mut self, direccion: Direccion) {
        self.estado = EstadoDron::Encamino(direccion.clone());

        match direccion {
            Direccion::Base => {
                self.trayectoria = Trayectoria::Recta(Recta::new(
                    self.posicion.clone(),
                    self.base_central.clone(),
                    self.unidad_de_movimiento * self.velocidad,
                ));
            }
            Direccion::Incidente(pos) => {
                self.trayectoria = Trayectoria::Recta(Recta::new(
                    self.posicion.clone(),
                    pos,
                    self.unidad_de_movimiento * self.velocidad,
                ));
            }
            Direccion::AreaDeOperaciones => {
                let pos_min = circunferencia::pto_mas_cercano_circunferencia(
                    self.base_central.lat,
                    self.base_central.long,
                    self.radio,
                    self.posicion.lat,
                    self.posicion.long,
                );
                self.trayectoria = Trayectoria::Recta(Recta::new(
                    self.posicion.clone(),
                    Posicion::new(pos_min.0, pos_min.1),
                    self.unidad_de_movimiento * self.velocidad,
                ));
            }
        }
    }

    pub fn set_estado_resolviendo(&mut self) {
        self.estado = EstadoDron::Resolviendo;
    }

    pub const fn get_estado(&self) -> &EstadoDron {
        &self.estado
    }
    pub const fn get_latitud(&self) -> f64 {
        self.posicion.lat
    }
    pub const fn get_longitud(&self) -> f64 {
        self.posicion.long
    }

    pub fn get_posicion(&self) -> Posicion {
        self.posicion.clone()
    }

    pub const fn get_id(&self) -> &i32 {
        &self.id
    }

    pub const fn get_bateria(&self) -> f64 {
        self.bateria
    }

    pub const fn get_angulo(&self) -> Option<&f64> {
        match &self.trayectoria {
            Trayectoria::Circunferencia(c) => Some(c.get_angulo()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use crate::dron::posicion::Posicion;

    use super::Dron;

    //se uso el siguiente valor para los test.
    //unidad de movimiento = 0.001.

    #[test]
    fn dron_se_crea_y_va_hacia_el_area_de_operacion() {
        let mut dron = Dron::new(1, 2.0, 1.0, 1.0, 100.0, 100.0, 1.0);

        while !dron.llegaste() {
            dron.mover();
            println!("Pos: {:#?}", dron.get_posicion());
        }

        assert_eq!(2.0, dron.get_latitud());
        assert_eq!(2.0, dron.get_longitud());
    }

    #[test]
    fn dron_setado_en_patrullar_tiene_el_angulo_inicial_correcto() {
        let mut dron = Dron::new(1, 2.0, 1.0, 1.0, 100.0, 100.0, 1.0);
        let precision = 0.0000001;
        dron.set_latitud(2.0);
        dron.set_longitud(2.0);

        dron.set_estado_patrullando();
        let a = dron.get_angulo().unwrap();
        assert!(&((0.0 * PI) - a).abs() <= &precision);

        dron.set_latitud(1.0);
        dron.set_longitud(3.0);

        dron.set_estado_patrullando();
        let a = dron.get_angulo().unwrap();
        assert!(&((0.5 * PI) - a).abs() <= &precision);

        dron.set_latitud(0.0);
        dron.set_longitud(2.0);

        dron.set_estado_patrullando();
        let a = dron.get_angulo().unwrap();
        assert!(&((PI) - a).abs() < &precision);

        dron.set_latitud(1.0);
        dron.set_longitud(1.0);

        dron.set_estado_patrullando();
        let a = dron.get_angulo().unwrap();
        assert!(&(((3.0 / 2.0) * PI) - a).abs() < &precision);
    }

    #[test]
    fn dron_recien_creado_patrulla() {
        let mut dron = Dron::new(1, 2.0, 1.0, 1.0, 250.0, 100.0, 1.0);
        let precision = 0.000001;

        while !dron.llegaste() {
            dron.mover();
        }

        if dron.llegaste() {
            dron.set_estado_patrullando();
            assert_eq!(2.0, dron.get_latitud());
            assert_eq!(2.0, dron.get_longitud());
            dron.mover();
            assert!((1.707106 - dron.get_latitud()).abs() < precision);
            assert!((2.707106 - dron.get_longitud()).abs() < precision);
            dron.mover();
            assert!((1.0 - dron.get_latitud()).abs() < precision);
            assert!((3.0 - dron.get_longitud()).abs() < precision);
            dron.mover();
            assert!((0.292893 - dron.get_latitud()).abs() < precision);
            assert!((2.707106 - dron.get_longitud()).abs() < precision);
            dron.mover();
            assert!((0.0 - dron.get_latitud()).abs() < precision);
            assert!((2.0 - dron.get_longitud()).abs() < precision);
            dron.mover();
            assert!((0.292893 - dron.get_latitud()).abs() < precision);
            assert!((1.292893 - dron.get_longitud()).abs() < precision);
            dron.mover();
            assert!((1.0 - dron.get_latitud()).abs() < precision);
            assert!((1.0 - dron.get_longitud()).abs() < precision);
            dron.mover();
            assert!((1.707106 - dron.get_latitud()).abs() < precision);
            assert!((1.292893 - dron.get_longitud()).abs() < precision);
            dron.mover();
            assert!((2.0 - dron.get_latitud()).abs() < precision);
            assert!((2.0 - dron.get_longitud()).abs() < precision);
            assert_eq!(&0.0, dron.get_angulo().unwrap());
        } else {
            assert!(false);
        }
    }

    #[test]
    pub fn dron_va_a_incidente_y_vuelve_al_area_de_operacion_ok() {
        let mut dron = Dron::new(1, 2.0, 1.0, 1.0, 250.0, 100.0, 1.0);
        let precision = 0.000001;

        dron.set_estado_en_camino(crate::dron::direccion::Direccion::Incidente(Posicion::new(
            3.0, 3.0,
        )));
        let mut pos_ida = Posicion::new(0.0, 0.0);

        while !dron.llegaste() {
            pos_ida = dron.mover();
        }

        let lat_incidente = pos_ida.lat;
        let long_incidente = pos_ida.long;

        dron.set_estado_resolviendo();
        let pos_resolviendo = dron.mover(); //no se deberia mover, o sea lo que devuelva debe ser (3.0, 3.0)

        dron.set_estado_en_camino(crate::dron::direccion::Direccion::AreaDeOperaciones);
        let mut pos_vuelta = Posicion::new(0.0, 0.0);
        while !dron.llegaste() {
            pos_vuelta = dron.mover();
        }

        assert_eq!(3.0, lat_incidente);
        assert_eq!(3.0, long_incidente);

        assert_eq!(3.0, pos_resolviendo.lat);
        assert_eq!(3.0, pos_resolviendo.long);

        assert!(1.894427 - pos_vuelta.lat <= precision);
        assert!(2.447213 - pos_vuelta.long <= precision);
    }

    #[test]
    pub fn dron_sin_bateria_vuelve_a_la_base_y_se_recarga() {
        let mut dron = Dron::new(1, 2.0, 1.0, 1.0, 250.0, 25.09, 1.0);
        let precision = 0.000001;

        while !dron.llegaste() {
            println!("Pos dron yendo a circunferencia: {:#?}", dron.mover());
        }
        dron.set_estado_patrullando();
        println!("Pos dron circunferencia: {:#?}", dron.mover());
        println!("Pos dron circunferencia: {:#?}", dron.mover());
        println!("Pos dron circunferencia: {:#?}", dron.mover());

        while dron.get_bateria() > 25.0 {
            dron.gastar_bateria();
        }
        let bat_min = dron.get_bateria();

        dron.set_estado_en_camino(crate::dron::direccion::Direccion::Base);
        let mut pos_base = Posicion::new(0.0, 0.0);
        while !dron.llegaste() {
            pos_base = dron.mover();
        }

        while !dron.cargado() {
            dron.cargar_bateria();
        }
        let bat = dron.get_bateria();

        assert!(25.0 - bat_min < precision);
        assert_eq!(1.0, pos_base.lat);
        assert_eq!(2.0, pos_base.long);
        assert_eq!(100.0, bat);
    }

    #[test]
    pub fn dron_toma_incidente_en_rango_ok() {
        let mut dron = Dron::new(1, 2.0, 1.0, 1.0, 250.0, 100.0, 1.0);

        while !dron.llegaste() {
            dron.mover();
        }
        dron.set_estado_patrullando();

        let afirmacion1 =
            dron.en_rango(2.5, 2.0) && dron.en_rango(2.5, 2.5) && !dron.en_rango(1.0, 0.5);
        println!("pos: {:#?}", dron.mover());
        println!("pos: {:#?}", dron.mover());
        let afirmacion2 =
            dron.en_rango(1.0, 3.5) && dron.en_rango(0.5, 3.0) && !dron.en_rango(2.5, 2.0);
        println!("pos: {:#?}", dron.mover());
        println!("pos: {:#?}", dron.mover());
        let afirmacion3 =
            dron.en_rango(-0.5, 2.0) && dron.en_rango(-0.5, 2.5) && !dron.en_rango(1.0, 3.5);
        println!("pos: {:#?}", dron.mover());
        println!("pos: {:#?}", dron.mover());
        let afirmacion4 =
            dron.en_rango(1.0, 0.5) && dron.en_rango(0.8, 0.8) && !dron.en_rango(-0.5, 2.0);
        assert!(afirmacion1 && afirmacion2 && afirmacion3 && afirmacion4);
    }
}
