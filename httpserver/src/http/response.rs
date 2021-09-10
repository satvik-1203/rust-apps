use super::status_code::StatusCode;
use std::io::Result as IOResult;
use std::io::Write;
use std::net::TcpStream;

pub struct Response {
  status_code: StatusCode,
  body: Option<String>,
}

// HTTP/1.1 200 OK

impl Response {
  pub fn New(status_code: StatusCode, body: Option<String>) -> Self {
    Response { status_code, body }
  }

  pub fn send(&self, tcp: &mut TcpStream) -> IOResult<()> {
    let body = match &self.body {
      Some(data) => data,
      None => "",
    };
    let (status, code) = &self.status_code.get_details();
    write!(tcp, "HTTP/1.1 {} {}\r\n\r\n{}", code, status, body)
  }
}
