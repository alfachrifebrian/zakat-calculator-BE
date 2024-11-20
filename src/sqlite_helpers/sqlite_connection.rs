use std::{env, error::Error};

pub struct SqliteConnection {
  connection_string: String,
  is_connected: bool,
  is_exist: bool,
}

impl SqliteConnection {
    pub fn first_run_check() -> Result<(Self), Box<dyn Error>> {
      let db_path = env::current_dir()?.join("zakat-calculator.db");
      if db_path.exists() && db_path.is_file() {
        return Ok(Self {
          connection_string: String::new(),
          is_connected: false,
          is_exist: true
        });
      } else {
        return Ok(Self {
          connection_string: String::new(),
          is_connected: false,
          is_exist: false
        });
      }
    }

    pub fn is_exist(&self) -> &bool {
      &self.is_exist
    }
}