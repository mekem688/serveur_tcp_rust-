use std::net::TcpStream;
use std::io::{Read, Write};
use std::io;
fn main() {
    // Se connecter au serveur
    let mut stream = TcpStream::connect("127.0.0.1:7878")
        .expect("Impossible de se connecter au serveur");
loop{
    // Message à envoyer
     println!("message a envoyer:  ");
    let mut message = String::new();
    io::stdin()
    .read_line(&mut message).unwrap();
    //let message = "Bonjour serveur !";
    stream.write(message.as_bytes()).unwrap();//.expect("Erreur lors de l'envoi");

    // Buffer pour recevoir la réponse
    let mut buffer = [0; 10012];
    let bytes_read = stream.read(&mut buffer).expect("Erreur lors de la lecture");

    // Afficher la réponse
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Réponse du serveur : {}", response);
}
}