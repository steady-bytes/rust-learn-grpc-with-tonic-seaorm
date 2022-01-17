use std::net::SocketAddr;
use log::{info};

use crate::api::user_management::users_v1::users_server::UsersServer;
use crate::cmd::users::rpc::{UsersRpcHandler};
use crate::cmd::users::controller::{UsersController};

///////////////////////////////////
/// ******* COMMAND ******* ///
///////////////////////////////////

/// Cmd - Is the application runtime that will be executed. The configuration of the process,
/// and it's runtime depencancies are held in the `Cmd` structure. The `default` method will build
/// the runtime with the default configurations to run. When ready to attach to the tcp socket `execute`
/// the command and the runtime should run "forever" and process requestes on the server.
pub struct Cmd {
    // TODO -> static configuration that should be populated by a configuration struct
    application_address: SocketAddr,
    handler: UsersRpcHandler,
}

impl Cmd {
  pub fn default() -> Cmd {
    env_logger::init();
    info!("starting up");

    // TODO -> add .env support for these configuration values, and generally
    // build out a better static configuration process.
    info!("reading configuration");
    let application_address = "[::]:50050".parse().unwrap();
    let relational_db_address = String::from("postgresql://services@localhost:26257/users");

    // initialize the application (order matters)
    info!("building dependancies");
    let svc = UsersController::default(relational_db_address);
    let hnd = UsersRpcHandler::default(svc);

    info!("runtime created");
    Cmd {
        application_address: application_address,
        handler: hnd,
    }
  }

  pub async fn execute(self) -> Result<(), Box<dyn std::error::Error>> {  
    let addr = self.application_address;

    // TODO -> add a graceful shutdown listener, and ways to control the dynamic
    // configuration of the server runtime.

    info!("running grpc server: {} on: {:?}", "Users", addr);

    // start the rpc server
    tonic::transport::Server::builder()
      .add_service(UsersServer::new(self.handler))
      .serve(addr)
      .await?;

    Ok(())
  }
}
