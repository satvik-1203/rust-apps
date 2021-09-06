use crate::http::request::Request;
use std::convert::{TryFrom, TryInto};
use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub struct Server {
  addr: String,
}

impl Server {
  pub fn new(addr: String) -> Self {
    Self { addr }
  }

  pub fn run(self) {
    println!("Listening on {}", self.addr);
    let listener = TcpListener::bind(&self.addr).unwrap();
    println!("{:?}", listener);
    loop {
      match listener.accept() {
        Ok((mut stream, _)) => {
          let mut buffer = [0; 1024];
          match stream.read(&mut buffer) {
            Ok(_) => match Request::try_from(&buffer[..]) {
              Ok(request) => {
                println!("{:#?}", request);
              }
              Err(e) => {
                println!("{:#?}", e)
              }
            },
            Err(err) => {
              println!("Error: {}", err)
            }
          }
        }
        Err(err) => println!("{}", err),
      }
    }
  }
}
