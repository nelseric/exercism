pub struct Robot {
  name: String
}

fn generate_name() -> String {
  "ABC123".to_string()
}

impl Robot {
  pub fn new() -> Robot {
    Robot { name: generate_name() }
  }
  pub fn name<'a>(&'a self) -> &'a str {
    // what does [..] mean?
    &self.name[..]
  }

  pub fn reset_name(&mut self) {
    self.name = generate_name();
  }
}