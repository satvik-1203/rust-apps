use enum_iterator::IntoEnumIterator;

#[derive(Debug, IntoEnumIterator)]
pub enum Options {
  SignUp,
  Login,
  AddTodo,
}

impl Options {
  pub fn message(&self) -> &str {
    match self {
      Self::SignUp => "Sign up",
      Self::AddTodo => "Add Todo",
      Self::Login => "Login",
    }
  }

  pub fn print() {
    for option in Options::into_enum_iter() {
      println!("{:?}", option);
    }
  }
}
