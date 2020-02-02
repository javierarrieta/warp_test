use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Customer {
  pub id: String,
  pub name: String,
}

impl Customer {
  pub fn new(id: String, name: String) -> Customer {
    Customer { id, name }
  }

  pub fn new_from_str(id: &str, name: &str) -> Customer {
    Customer { id: id.to_owned(), name: name.to_owned() }
  }
}