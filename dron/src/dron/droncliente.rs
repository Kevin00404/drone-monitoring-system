use crate::dron::posicion::Posicion;
use cliente::cliente::client::NatsClient;
use cliente::cliente::iclient::INatsClient;
use cliente::cliente::user::User;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use crate::dron::direccion::Direccion;
use crate::dron::dron::Dron;
use crate::dron::errores::ErroresDron;
use crate::dron::estadodron::EstadoDron;

#[derive(Deserialize, Debug)]
/// Provee la configuración necesaria para correr el cliente de dron
pub struct Config {
    pub address_client: String,
    pub pub_drones: String,
    pub sub_incidentes: String,
    pub username: String,
    pub password: String,
    pub velocidad: f64,
}

impl Config {
    fn from_file(file_path: &str) -> Result<Self, Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }
}

/// Representa un dron con su cliente NATS
pub struct Droncliente {
    pub dron: Arc<Mutex<Dron>>,
    pub client: Arc<Mutex<Box<dyn INatsClient + Send>>>, // Cliente NATS para la comunicación
    subs: HashMap<String, usize>,
    detenerse: Arc<Mutex<bool>>,
    _id_incidente: Arc<Mutex<i32>>,
    config: Config,
}

#[allow(clippy::mutex_integer)]
#[allow(clippy::significant_drop_in_scrutinee)]
impl Droncliente {
    /// Inicializa el dron cliente
    fn init(
        nats: Box<dyn INatsClient + Send>,
        id: i32,
        longitud_base: f64,
        latitud_base: f64,
        alcance: f64,
        bateria: f64,
        config: Config,
    ) -> Result<Self, Error> {
        let nats_lockeado: Arc<Mutex<Box<dyn INatsClient + Send>>> = Arc::new(Mutex::new(nats));
        let nats_lockeado_save = nats_lockeado.clone();
        let nats_lockeado_now = nats_lockeado.clone();
        let id_incidente = Arc::new(Mutex::new(-1));
        let detenerse: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

        let velocidad = config.velocidad;
        let pub_drones = config.pub_drones.clone();
        let sub_incidentes = config.sub_incidentes.clone();

        let dron: Arc<Mutex<Dron>> = Arc::new(Mutex::new(Dron::new(
            id,
            longitud_base,
            latitud_base,
            alcance,
            velocidad,
            bateria,
            40.0 * alcance,
        )));

        publish_drones(
            & nats_lockeado_save,
            "enCaminoAreaOp",
            &id,
            None,
            pub_drones.as_str(),
            vec![latitud_base, longitud_base, bateria],
        )?;
        let dron_lock = dron.clone();

        let id_incidente_clone = id_incidente.clone();
        let mut id_incidente_clone2 = id_incidente.clone();

        let mut nats_safe = nats_lockeado_now
            .lock()
            .map_err(|_| ErroresDron::InternalError("Lock envenenado".to_string()))?;

        let jetstream = "JetStream-Incidentes";
        let consumer_name = &format!("IncidentesCosumer-{}", id);
        let delivery_subject = &format!("incidentes_delivery-{}", id);

        nats_safe.create_stream(sub_incidentes.as_str(), jetstream)?;
        let sub_id = nats_safe.create_and_consume(
        jetstream,
        consumer_name,
        delivery_subject,
        Box::new(move |payload| {
            let mut iter = payload.0.split_whitespace();
            println!("Payload INCIDENTE EN DRONE: {}", payload.0);
            let mut contador_todo_correcto = 0;

            let mut id = 0;
            if let Some(some_id) = iter.next() {
                if let Ok(ok_id) = some_id.parse::<i32>(){
                    contador_todo_correcto +=1;
                    id = ok_id;
                }
            };
            let mut latitud_incidente = 0.0;
            if let Some(some_lat) = iter.next() {
                if let Ok(ok_lat) = some_lat.parse::<f64>(){
                    contador_todo_correcto +=1;
                println!("latitud: {}", ok_lat);

                    latitud_incidente = ok_lat;
                }
            };

            let mut longitud_incidente = 0.0;
            if let Some(some_long) = iter.next() {
                if let Ok(ok_long)  = some_long.parse::<f64>(){
                    contador_todo_correcto +=1;
                    longitud_incidente = ok_long;
                }
            };

            let mut estado = "";
            if let Some(some_estado) = iter.next(){
                contador_todo_correcto += 1;
                estado = some_estado;
            }

            if contador_todo_correcto != 4{
                return;
            }

            if let Ok(mut drone)= dron_lock.lock(){
                if estado == "pendiente" && drone.disponible(latitud_incidente, longitud_incidente){
                    drone.set_estado_en_camino(Direccion::Incidente(Posicion::new(latitud_incidente, longitud_incidente)));
                    if let Ok(mut id_inc) = id_incidente_clone.lock(){
                        *id_inc = id;
                    }
                }else if estado == "pendiente" && drone.poca_bateria() {
                    drone.set_estado_en_camino(Direccion::Base);

                }else if estado == "cancelado"{ //DEBERIA VOLVER A LA BASE O AREA DE OPERACION.

                    if let Ok(mut id_incidente_dron) = id_incidente_clone.lock(){
                        if *id_incidente_dron == id {

                            if drone.poca_bateria(){
                                drone.set_estado_en_camino(Direccion::Base);
                            }else{
                                drone.set_estado_en_camino(Direccion::AreaDeOperaciones);
                            }
                            *id_incidente_dron = -1;
                        }
                    }

                }else if estado == "activo" {
                    let mut vec = Vec::new();
                    if let Some(some_id1) = iter.next(){
                        if let Ok(ok_id1) = some_id1.parse::<i32>(){
                            vec.push(ok_id1);
                        }
                    }

                    if let Some(some_id2) = iter.next(){
                        if let Ok(ok_id2) = some_id2.parse::<i32>(){
                            vec.push(ok_id2);
                        }
                    }

                    if vec.len() != 2{
                        println!("Error al parsear los id de los drones que estan resolviendo el incidente");
                        return;
                    }

                    if let Ok(mut id_incidente_dron) = id_incidente_clone.lock(){
                        if *id_incidente_dron == id && !drone.match_id(vec){
                            match drone.get_estado() {
                                EstadoDron::Encamino(Direccion::Incidente(_)) => {
                                    drone.set_estado_en_camino(Direccion::AreaDeOperaciones);
                                },
                                EstadoDron::Resolviendo => {
                                    drone.set_estado_en_camino(Direccion::AreaDeOperaciones);
                                }
                                _=> {
                                    println!("Error no se puede estar siguiendo un incidente y el estado no ser: \"En camino al incidente\" O \"Resolviendo\"");
                                    return;
                                },
                            }
                            *id_incidente_dron = -1; // como ya no puedo resolver el incidente, paso mi id de incidente a -1.
                        }
                    }else{
                        println!("Error Interno: Lock envenenado");
                        return;
                    }
                }else {
                    println!("Error Interno: Lock envenenado");
                    return;
                }
            } else {
                println!("Error Interno: Lock envenenado");
            }
        }),
    )?;
        drop(nats_safe);

        let mut sub_hash: HashMap<String, usize> = HashMap::new();
        sub_hash.insert(config.sub_incidentes.to_string(), sub_id);

        let detenerse_th = detenerse.clone();
        let dron_th = dron.clone();
        let nats_th = nats_lockeado.clone();

        let builder = thread::Builder::new().name(format!("thread-dron-id:{}", id));

        let _ = builder.spawn(move || {
            loop {
                if let Ok(parar) = detenerse_th.lock() {
                    if *parar {
                        drop(parar);
                        break;
                    } else {
                        drop(parar);
                    }
                }

                if let Ok(mut drone) = dron_th.lock() {
                    let pos;
                    match drone.get_estado() {
                        EstadoDron::Encamino(_) => {
                            //en camino a la base, incidente o area de op.

                            if drone.poca_bateria() && matches!(drone.get_estado(), EstadoDron::Encamino(Direccion::AreaDeOperaciones)) {
                                drone.set_estado_en_camino(Direccion::Base);
                            }

                            pos = drone.mover();
                            drone.gastar_bateria();

                            if drone.llegaste() {
                                match drone.get_estado() {
                                    EstadoDron::Encamino(Direccion::Incidente(_)) => {
                                        drone.set_estado_resolviendo()
                                    }
                                    EstadoDron::Encamino(Direccion::AreaDeOperaciones) => {
                                        drone.set_estado_patrullando()
                                    }
                                    EstadoDron::Encamino(Direccion::Base) => {
                                        drone.set_estado_recargando()
                                    }
                                    _ => (),
                                }
                            }
                        }
                        EstadoDron::Recargando => {
                            drone.cargar_bateria();
                            pos = drone.get_posicion();

                            if drone.cargado() {
                                drone.set_estado_en_camino(Direccion::AreaDeOperaciones);
                            }
                        }
                        EstadoDron::Resolviendo => {
                            drone.gastar_bateria();
                            pos = drone.get_posicion();
                        }
                        EstadoDron::Patrullando => {
                            if drone.poca_bateria() {
                                drone.set_estado_en_camino(Direccion::Base);
                            }

                            pos = drone.mover();
                            drone.gastar_bateria();
                        }
                    }

                    match drone.get_estado() {
                        EstadoDron::Resolviendo => {
                            if let Err(e) = publish_drones(
                                & nats_th,
                                &drone.get_estado().to_string(),
                                drone.get_id(),
                                Some(&mut id_incidente_clone2),
                                pub_drones.as_str(),
                                vec![pos.lat, pos.long, drone.get_bateria()]
                            ) {
                                println!("{}", e);
                            }
                        }
                        _ => {
                            if let Err(e) = publish_drones(
                                & nats_th,
                                &drone.get_estado().to_string(),
                                drone.get_id(), 
                                None,
                                pub_drones.as_str(),
                                vec![pos.lat, pos.long, drone.get_bateria()]
                            ) {
                                println!("{}", e);
                            }
                        }
                    }
                    thread::sleep(Duration::from_secs(1));
                }
            }
        })?;

        //movimiento.join().map_err(|_| ErroresDron::InternalError("Error al hacer el join handle".to_string()))?;

        // Retorna una instancia de Dron con las suscripciones ya hechas
        Ok(Droncliente {
            dron,
            client: nats_lockeado_save,
            subs: sub_hash,
            detenerse,
            _id_incidente: id_incidente,
            config,
        })
    }

    /// Elimina el dron
    pub fn eliminar(&mut self) -> Result<(), ErroresDron> {
        if let Ok(mut cliente) = self.client.lock() {
            if let Some(id) = self.subs.get_mut(self.config.sub_incidentes.as_str()) {
                let _ = cliente.unsubscribe(*id, None);
                let mut parar = self
                    .detenerse
                    .lock()
                    .map_err(|_| ErroresDron::InternalError("Lock envenenado".to_string()))?;
                *parar = true;
                drop(parar);
            }
        }
        Ok(())
    }

    ///Inicializa app
    pub fn new(
        id: i32,
        latitud_base: f64,
        longitud_base: f64,
        alcance: f64,
        bateria: f64,
    ) -> Result<Self, Error> {
        let config = match Config::from_file("conf.json") {
            Ok(config) => config,
            Err(_) => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Error al cargar conf.json",
                ));
            }
        };

        let writer = TcpStream::connect(&config.address_client)?; // Conecta el escritor al servidor NATS p[ara escribir en channel a Camara
        let reader = writer.try_clone()?; // Clona el escritor para usarlo como lector de Camara y de Sist de Monitoreo

        //nuevo cliente NATS
        let nats = NatsClient::new(
            writer,
            reader,
            "logs.txt",
            Some(User::new(config.username.clone(), config.password.clone())),
        )?;

        Self::init(
            Box::new(nats),
            id,
            longitud_base,
            latitud_base,
            alcance,
            bateria,
            config,
        )
    }
}

/// Envia un mensaje al servidor con la información del dron
fn publish_drones(
    client_lockeado: &Arc<Mutex<Box<dyn INatsClient + Send>>>,
    estado: &str,
    id: &i32,
    id_inc: Option<&mut Arc<Mutex<i32>>>,
    pub_drones: &str,
    numeros: Vec<f64>,
) -> Result<(), ErroresDron> {

    let lat: f64 = numeros[0];
    let long: f64 = numeros[1];
    let bateria: f64 = numeros[2];

    if let Ok(mut client) = client_lockeado.lock() {
        let payload = if let Some(id_inc_lock) = id_inc {
            if let Ok(id_incidente) = id_inc_lock.lock() {
                format!(
                    "{} {} {} {} {} {}",
                    lat,
                    long,
                    estado,
                    id,
                    bateria,
                    id_incidente,
                )
            } else {
                return Err(ErroresDron::InternalError("Lock envenenado".to_string()));
            }
        } else {
            format!(
                "{} {} {} {} {}",
                lat,
                long,
                estado,
                id,
                bateria,
            )
        };

        client
            .publish(pub_drones, Some(&payload), None)
            .map_err(|_| ErroresDron::InternalError("Al hacer publish de dron".to_string()))?;
        Ok(())
    } else {
        Err(ErroresDron::InternalError("Lock envenenado".to_string()))
    }
}
