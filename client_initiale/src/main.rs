use std::net::TcpStream;
use std::io::Write;
fn main() {
  let mut stream =
  TcpStream::connect("127.0.0.1:8000").unwrap();

  let mess=stream.write(b"bonjour serveur").unwrap();

   stream.write(b"salut les developpeurs ").unwrap();

   println!("message envoyee .... {}",mess);
}
