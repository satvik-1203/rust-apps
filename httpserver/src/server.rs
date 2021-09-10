use crate::http::{request::Request, response::Response, status_code::StatusCode};

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
                let res = Response::New(StatusCode::Ok, Some("<h1>WOW Working</h1>".to_string()));
                res.send(&mut stream);
              }
              Err(e) => {
                let res =
                  Response::New(StatusCode::BadRequest, Some("<h1>Failed</h1>".to_string()));
                res.send(&mut stream);
              }
            },
            Err(_) => {}
          }
        }
        Err(_) => {}
      }
    }
  }
}
