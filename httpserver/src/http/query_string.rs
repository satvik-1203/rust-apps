use std::collections::HashMap;

// a=3?b?c===?b=3
#[derive(Debug)]
pub struct QueryString<'buff> {
  data: HashMap<&'buff str, Value<'buff>>,
}
#[derive(Debug)]
pub enum Value<'buff> {
  Single(&'buff str),
  Multiple(Vec<&'buff str>),
}

impl<'buff> QueryString<'buff> {
  fn get(&self, key: &str) -> Option<&Value> {
    self.data.get(key)
  }
}

impl<'buff> From<&'buff str> for QueryString<'buff> {
  fn from(string: &'buff str) -> Self {
    let mut data = HashMap::new();
    for mut query in string.split("?") {
      let mut key = query;
      let mut value = "";
      if let Some(i) = query.find("=") {
        key = &query[..i];
        value = &query[i + 1..];
      }
      data
        .entry(key)
        .and_modify(|exist| match exist {
          Value::Multiple(vec) => vec.push(value),
          Value::Single(prev_str) => {
            *exist = Value::Multiple(vec![value, prev_str]);
          }
        })
        .or_insert(Value::Single(value));
    }
    QueryString { data }
  }
}
