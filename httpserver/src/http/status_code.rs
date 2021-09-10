#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
  Ok = 200,
  NotFound = 404,
  BadRequest = 400,
}

impl StatusCode {
  pub fn get_details(&self) -> (String, u16) {
    let message = match self {
      Self::Ok => "OK",
      Self::NotFound => "Not Found",
      Self::BadRequest => "Bad Request",
    };
    let code = *self as u16;

    (message.to_string(), code)
  }
}
