use std::convert::{From, TryFrom};
use std::fmt::Formatter as FmtFormatter;
use std::io::{stdin, stdout, Error, Write};

pub fn response() -> Result<u16, InputError> {
  let mut buff = String::new();

  print!("Please enter your option: ");
  stdout().flush();

  stdin().read_line(&mut buff)?;

  let option = buff.trim().parse::<u16>()?;

  if (option > 6 || option < 0) {
    return Err(InputError::InvalidOption);
  }

  return Ok(option);
}

pub enum InputError {
  EncodingFailed,
  NotAnInteger,
  InvalidOption,
}

impl From<Error> for InputError {
  fn from(_: Error) -> Self {
    Self::EncodingFailed
  }
}

impl InputError {
  pub fn message(&self) -> &str {
    match self {
      Self::EncodingFailed => "failed Encoding",
      Self::NotAnInteger => "Input not an Integer",
      Self::InvalidOption => "Invalid Option",
    }
  }
}

impl From<std::num::ParseIntError> for InputError {
  fn from(_: std::num::ParseIntError) -> Self {
    Self::NotAnInteger
  }
}
