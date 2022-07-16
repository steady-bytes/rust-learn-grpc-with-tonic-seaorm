pub mod users;

use clap::{Parser, Subcommand, ValueEnum, Args};
use async_trait::async_trait;
use std::net::SocketAddr;
use log::{info};
use sea_orm::{ActiveModelTrait, DbErr};

// gen imports
use crate::api::user_management::users_v1::users_server::UsersServer;
use crate::api::user_management::users_v1::users_client::UsersClient;

use crate::cmd::users::handler::{UsersHandler};
use crate::cmd::users::controller::{UsersController};

///////////////////////////////////
/// ******* COMMAND ******* ///
///////////////////////////////////

/// Cmd - Is the application runtime that will be executed. The configuration of the process,
/// and it's runtime depencancies are held in the `Cmd` structure. The `default` method will build
/// the runtime with the default configurations to run. When ready to attach to the tcp socket `execute`
/// the command and the runtime should run "forever" and process requestes on the server.
pub struct Cmd {
    application_address: Option<SocketAddr>,
    handler: Option<UsersHandler>,
    client: Option<UsersClient<tonic::transport::Channel>>,
}

type CmdResult<T> = std::result::Result<T, CmdError>;

#[derive(Debug, Clone)]
pub struct CmdError;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    runtime: Runtime,
}


#[derive(Subcommand, Debug)]
enum Runtime {
  Server,
  Client(ClientMethod),
}

#[derive(Args, Clone, Debug)]
struct ClientMethod {
  #[clap(subcommand)]
  method: Option<RpcMethod>,
}

#[derive(Debug, Clone, Subcommand)]
enum RpcMethod {
  Signup{
    #[clap(value_parser)]
    name: Option<String>,
  },
  GetById,
}

impl Cmd {
  pub fn new () -> Cmd {
    Cmd { application_address: None, handler: None, client: None}
  }

  pub async fn default(&self) -> CmdResult<Cmd> {
    env_logger::init();
    info!("starting up");

    let args = Cli::parse();

    info!("{args:?}");

    match args.runtime {
      Runtime::Server => {
        Ok(self.build_server().unwrap())
      },
      Runtime::Client(client_method) => {
        let m = client_method.method.unwrap();

        info!("nested command: {m:?}");

        Ok(self.build_client().await.unwrap())
      }
    }
  }

  pub fn build_server(&self) -> CmdResult<Cmd> {
    // TODO -> add .env support for these configuration values, and generally
    // build out a better static configuration process.
    info!("reading server configuration");
    let application_address = Some("[::]:50050".parse().unwrap());
    let relational_db_address = String::from("postgres://services@localhost:26257/users");

    // initialize the application (order matters)
    info!("building dependancies");
    let svc = UsersController::default(relational_db_address);

    match svc {
      Some(s) => {
        let hnd = UsersHandler::default(s);
        info!("runtime created");
        Ok(Cmd {
            application_address: application_address,
            handler: Some(hnd),
            client: None,
        })
      },
      None => { 
        info!("db did not connect");
        Err(CmdError)
      }
    }
  }

  pub async fn build_client(&self) -> CmdResult<Cmd> {
    info!("client");

    let client = UsersClient::connect("http://[::]:50050").await.unwrap();
 
    Ok(Cmd {
      application_address: None,
      handler: None,
      client: Some(client),
    })
  }

  pub async fn execute(self) -> Result<(), Box<dyn std::error::Error>> {  
    let addr = self.application_address;

    // TODO -> add a graceful shutdown listener, and ways to control the dynamic
    // configuration of the server runtime.

    info!("running grpc server: {} on: {:?}", "Users", addr);

    // start the rpc server
    tonic::transport::Server::builder()
      .add_service(UsersServer::new(self.handler.unwrap()))
      .serve(addr.unwrap())
      .await?;

    info!("rpc service has started");

    Ok(())
  }
}

/// SingularCrud
#[async_trait]
pub trait SingularCrud {
  /// Entity is the representation of what table, and it's schema. Crud operations
  /// are implemented for this type, and it's the return value for each method.
  type Entity: ActiveModelTrait;

  /// Often the input values to an entity are not the same at the returned model.
  /// providing the `InputInputFormFields` allows for this flexibility.
  type InputFormFields: ActiveModelTrait;
  
  /// Create one `Entity` given a set of `InputInputFormFields`.
  async fn create(&self, ff: Self::InputFormFields) -> Result<Self::Entity, DbErr>;

  /// Read one `Entity` given a set of `InputInputFormFields` that contains the uuid
  /// of the desired `Entity`
  async fn read(&self, ff: Self::InputFormFields) -> Result<Self::Entity, DbErr>;

  /// Update one `Entity` with the new values in the `InputFormFields` it's recommended
  /// to use `Options` for the key/values in `InputFormFields` so that on the non-None
  /// values are changed in the `Entity`
  async fn update(&self, ff: Self::InputFormFields) -> Result<Self::Entity, DbErr>;

  /// Delete one `Entity` if found using the `InputFormFields` id.
  async fn delete(&self, ff: Self::InputFormFields) -> Result<bool, DbErr>;
}
