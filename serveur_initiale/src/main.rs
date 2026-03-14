use std::net::TcpListener;
use std::io::Read;
fn main() {
   let listener = 
   TcpListener::bind("127.0.0.1:8000").unwrap();

   println!("serveur en ecoute....");
   for stream in listener.incoming(){
    let mut stream =
    stream.unwrap();
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();

    println!("{}",String::from_utf8_lossy(&buffer));
   }

}
