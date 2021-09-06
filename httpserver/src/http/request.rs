use super::methods::{Method, MethodError};
use super::query_string::QueryString;
use std::convert::{From, TryFrom};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buff> {
  path: &'buff str,
  query_string: Option<QueryString<'buff>>,
  method: Method,
}

// GET / HTTP/1.1

impl<'buff> TryFrom<&'buff [u8]> for Request<'buff> {
  type Error = ParseError;
  fn try_from(value: &'buff [u8]) -> Result<Self, Self::Error> {
    let mut string = str::from_utf8(value)?;
    let (method, string) = get_next_word(&string).ok_or(ParseError::InvalidRequest)?;
    let (mut path, string) = get_next_word(string).ok_or(ParseError::InvalidRequest)?;
    let (protocol, _) = get_next_word(string).ok_or(ParseError::InvalidRequest)?;
    if (protocol != "HTTP/1.1") {
      return Err(ParseError::InvalidProtocol);
    }
    let method: Method = method.parse()?;

    let mut query_string = None;

    if let Some(i) = path.find("?") {
      query_string = Some(QueryString::from(&path[i + 1..]));
      path = &path[..i];
    }

    Ok(Request {
      path,
      query_string,
      method,
    })
  }
}

fn get_next_word(string: &str) -> Option<(&str, &str)> {
  for (index, ch) in string.chars().enumerate() {
    if (ch == ' ' || ch == '\r') {
      return Some((&string[..index], &string[index + 1..]));
    }
  }
  None
}

impl From<MethodError> for ParseError {
  fn from(_: MethodError) -> Self {
    Self::InvalidMethod
  }
}

impl From<Utf8Error> for ParseError {
  fn from(_: Utf8Error) -> Self {
    Self::InvalidEncoding
  }
}

pub enum ParseError {
  InvalidProtocol,
  InvalidEncoding,
  InvalidRequest,
  InvalidMethod,
}

impl ParseError {
  fn message(&self) -> &str {
    match self {
      Self::InvalidEncoding => "InvalidEncoding",
      Self::InvalidMethod => "InvalidMethod",
      Self::InvalidProtocol => "InvalidProtocol",
      Self::InvalidRequest => "InvalidRequest",
    }
  }
}

impl Debug for ParseError {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "Failed with: {}", self.message())
  }
}

impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "Failed with: {}", self.message())
  }
}
