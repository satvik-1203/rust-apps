use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
  GET,
  PUT,
  POST,
  DELETE,
}

impl FromStr for Method {
  type Err = MethodError;

  fn from_str(method: &str) -> Result<Self, Self::Err> {
    match method {
      "GET" => Ok(Method::GET),
      "PUT" => Ok(Method::PUT),
      "POST" => Ok(Method::POST),
      "DELETE" => Ok(Method::DELETE),
      _ => Err(MethodError),
    }
  }
}

pub struct MethodError;
