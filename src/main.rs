#[macro_use]
extern crate diesel;
extern crate validator;
extern crate log;

use crate::cmd::root::{Cmd};

mod api;
mod cmd;
mod dao;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Cmd::default().execute().await?;

    Ok(())
}
