extern crate validator;
extern crate log;

use crate::cmd::{Cmd};

mod api;
mod cmd;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  match Cmd::new().default().await {
    Ok(x) => {
      x.execute().await?;
      Ok(())
    },
    Err(_e) => Ok(())
  }
}
