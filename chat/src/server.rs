use std::net::{SocketAddr, TcpListener};

pub struct Server<'addr> {
  addr: &'addr str,
}

impl<'addr> Server<'addr> {
  pub fn New(addr: &'addr str) -> Self {
    Self { addr }
  }
  pub fn Run(&self) {
    println!("Listening on addr: {}", self.addr);
    let listener = TcpListener::bind(self.addr).unwrap();
    // let data = listener.accept().unwrap();
    // println!("{:#?}", data);
    loop {
      match listener.accept() {
        Ok((stream, addr)) => {
          println!("New client on addr: {:?}", addr)
        }
        Err(e) => println!("{:?}", e),
      }
    }
  }
}
