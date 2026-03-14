use std::net::{TcpListener, TcpStream}; // Pour créer le serveur et gérer les connexions
use std::io::{Read, Write}; // Pour lire et écrire sur le stream

fn handle_client(mut stream: TcpStream) {
    // Cette fonction traite un client unique
    let mut buffer = [0; 512]; // Buffer pour recevoir les données (512 octets max)
    
    // Lire les données envoyées par le client
    let bytes_read = stream.read(&mut buffer).expect("Erreur lors de la lecture du client");

    // Convertir les octets lus en chaîne pour affichage
    let received = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Message reçu : {}", received);

    // Répondre au client
    let response = format!("Serveur a reçu : {}", received);
    stream.write(response.as_bytes()).expect("Erreur lors de l'envoi de la réponse");
}

fn main() {
    // Créer le listener TCP sur l'adresse locale et le port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Impossible de lier le serveur au port");

    println!("Serveur en écoute sur 127.0.0.1:7878...");

    // Boucle infinie pour accepter les clients
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Nouvelle connexion : {}", stream.peer_addr().unwrap());
                handle_client(stream); // Traiter le client
            }
            Err(e) => {
                println!("Erreur de connexion : {}", e);
            }
        }
    }
}