pub struct Vars {
  pub stage: i8,
}

impl Vars {
  pub fn new() -> Self {
    Self {
      stage: 0,
    }
  }
}

pub struct Config {
  pub vars: Vars,
}

impl Config {
  pub fn new() -> Self {
      Self {
          vars: Vars::new(),
      }
  }
}