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

pub struct MenuConfig {
  pub vars: Vars,
}

impl MenuConfig {
  pub fn new() -> Self {
    Self {
      vars: Vars::new(),
    }
  }
}