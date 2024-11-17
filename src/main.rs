#[macro_use]
extern crate rocket;

use rocket::State;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::thread;
use config::Config;
use serde::Deserialize;

use log::{info, warn, error};

const SERVER_ADDRESS: &str = "event.nationsglory.fr:59001";
const REMOVE_WAITLIST: &str = "MESSAGE socket REMOVE_WAITINGLIST";

#[derive(Debug, Deserialize)]
struct AppConfig {
    auth_string: String,
    address: String,
    port: u16,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();

    let (tx, rx) = std::sync::mpsc::channel::<String>();

    let settings = Config::builder()
        .set_default("port", 8000).unwrap()
        .set_default("address", "127.0.0.1").unwrap()
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap();

    let config: AppConfig = settings.try_deserialize().unwrap();

    if config.auth_string.is_empty() {
        error!("Le champ auth_string est vide dans le fichier de configuration. Merci de le spécifier !");
        return Ok(());
    }

    let auth_string = config.auth_string.clone();
    thread::spawn(move || listen_to_server(rx, auth_string));

    rocket::build()
        .manage(tx)
        .mount("/", routes![connect])
        .configure(rocket::Config::figment().merge(("port", &config.port)).merge(("address", &config.address)))
        .launch()
        .await?;

    Ok(())
}

#[get("/connect/<server>")]
async fn connect(server: &str, tx: &State<Sender<String>>) -> String {
    if server.is_empty() {
        return "Le serveur ne peut-être vide".to_string();
    }

    tx.send(server.to_string()).unwrap();
    format!("Message envoyé : {}", server)
}

fn listen_to_server(rx: std::sync::mpsc::Receiver<String>, auth_string: String) {
    let stream = TcpStream::connect(SERVER_ADDRESS);
    let stream = match stream {
        Ok(s) => {
            info!("Connecté au serveur");
            Some(s)
        }
        Err(e) => {
            error!("Erreur de connexion au serveur: {}", e);
            None
        }
    };

    let mut is_auth = false;

    if let Some(mut tcp_stream) = stream {
        let reader = BufReader::new(tcp_stream.try_clone().expect("Erreur de clonage du flux"));

        for line in reader.lines() {
            match line {
                Ok(server_message) => {
                    if !server_message.starts_with("PING_AND_DATA") {
                        info!("Message reçu du serveur : {}", server_message);

                        if server_message.starts_with("SUBMITNAME") {
                            writeln!(tcp_stream, "{}", auth_string).unwrap();
                        }

                        if server_message.starts_with("NAMEACCEPTED") {
                            is_auth = true;
                        }
                    }
                }
                Err(e) => {
                    error!("Erreur lors de la lecture du message : {}", e);
                    break;
                }
            }

            if let Ok(server) = rx.try_recv() {
                if is_auth {
                    let message = format!("MESSAGE socket ADD_WAITINGLIST {}", server);
                    writeln!(tcp_stream, "{}", message).unwrap();
                    writeln!(tcp_stream, "{}", REMOVE_WAITLIST).unwrap();
                }
            }
        }
    }

    warn!("Connexion fermée par le serveur.");
}
