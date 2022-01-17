use log::{debug, warn, info};
use tonic::{Request, Response, Status};
use validator::{Validate};
use crate::cmd::crud::SingularCrud;
use crate::cmd::users::model;
use crate::cmd::users::controller::{UsersController}; 
use crate::api::user_management::users_v1::{
  users_server::Users,
  { 
    HelloUserRequest, 
    HelloUserResponse, 
    SignupUserRequest, 
    SignupUserResponse, 
    GetUserByIdRequest,
    GetUserByIdResponse,
    UpdateUserRequest,
    UpdateUserResponse,
    DeleteByIdRequest,
    DeleteByIdResponse
  },
};


///////////////////////////////////
/// ********* HANDLER ********* ///
///////////////////////////////////

pub struct UsersRpcHandler {
  service: UsersController,
}

impl UsersRpcHandler {
  pub fn default(service: UsersController) -> Self {
    UsersRpcHandler {
      service: service,
    }
  }
}

#[tonic::async_trait]
impl Users for UsersRpcHandler {
    async fn signup(
        &self,
        request: Request<SignupUserRequest>,
    ) -> Result<Response<SignupUserResponse>, Status> {
      info!("signup request: {:?}", request);

      // extract request values into `UserForm`
      let new_user = model::UserForm::from(request.into_inner().model.unwrap()); 

      // validate 
      match new_user.validate() {
        Ok(v) => {
          debug!("validation output: {:?}", v);
        },
        Err(e) => {
          warn!("validation error: {:?}", e);
          let res = Status::new(tonic::Code::InvalidArgument, e.to_string());
          return Err(res)
        } 
      }
      
      // todo -> policy enforcement

      // execute create user business logic found in the service layer
      match self.service.create(new_user) {
        Ok(user) => {
          let res = SignupUserResponse {id: user.id.to_string()};
          debug!("response: {:?}", res);
          Ok(Response::new(res))
        }
        Err(e) => {
          let res = Status::new(tonic::Code::Internal, e);
          warn!("user signup failed: {:?}", res);
          Err(res)
        }
      }
    }

    async fn get_by_id(
      &self,
      request: Request<GetUserByIdRequest>,
    ) -> Result<Response<GetUserByIdResponse>, Status> {
      info!("get_by_id request: {:?}", request);

      // extract request values into `UserForm`
      let user_reader = model::UserForm::new(uuid::Uuid::parse_str(&request.into_inner().id).unwrap());

      // validate 
      match user_reader.validate() {
        Ok(v) => {
          debug!("validation output: {:?}", v);
        },
        Err(e) => {
          warn!("new user validation error: {:?}", e);

          let res = Status::new(tonic::Code::InvalidArgument, e.to_string());
          return Err(res)
        } 
      }

      // todo -> policy enforcement

      // call the service layer to execute the business logic for this request
      match self.service.read(user_reader) {
        Ok(res) => {
          debug!("response: {:?}", res);
          Ok(Response::new(GetUserByIdResponse{model: Some(res.into())}))
        },
        Err(e) => {
          warn!("could not read user: {:?}", e);
          Err(Status::new(tonic::Code::Internal, e))
        }
      }
    }

    async fn update_by_id(
      &self,
      request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserResponse>, Status> {
      debug!("update_by_id: {:?}", request);

      // extract reqquest values into `User` 
      let req = request.into_inner(); 
      let user = model::User::from(req.model.unwrap());

      // validate
      match user.validate() {
        Ok(v) => {
          debug!("validation output: {:?}", v);
        },
        Err(e) => {
          warn!("validation error: {:?}", e);
          let res = Status::new(tonic::Code::InvalidArgument, e.to_string());
          return Err(res)
        } 
      }

      // todo -> policy enforcement

      // create `UserForm` to update values in the database
      match model::UserForm::from_user_and_update_fields(user, req.update_fields) {
        Ok(uf) => {
          debug!("UserForm created from update_fields: {:?}", uf);
          match self.service.update(uf) {
            Ok(u) => {
              debug!("UserForm created {:?}", u);
              let res = UpdateUserResponse {model: Some(u.into())};
              Ok(Response::new(res))
            }
            Err(e) => {
              warn!("failed to update user entity: {:?}", e);
              let res = Status::new(tonic::Code::Internal, e);
              Err(res)
            },
          }
        },
        Err(e) => {
          warn!("failed to create UserForm from update_fields: {:?}", e);
          let res = Status::new(tonic::Code::Internal, e);
          return Err(res)
        },
      }  
    }

    async fn delete_by_id(
      &self,
      request: Request<DeleteByIdRequest>,
    ) -> Result<Response<DeleteByIdResponse>, Status> {
      debug!("delete_by_id request: {:?}", request);
      
      // extract request values into `UserForm`
      let user_deleter = model::UserForm::new(uuid::Uuid::parse_str(&request.into_inner().id).unwrap());

      // todo -> Policy Enforement

      match self.service.delete(user_deleter) {
        Ok(u) => {
          debug!("deleted user with id: {:?}", u.id.to_string());
          let res = DeleteByIdResponse{id: u.id.to_string()};
          Ok(Response::new(res))
        },
        Err(e) => {
          warn!("failed to delete user: {:?}", e);
          let res = Status::new(tonic::Code::Internal, e);
          Err(res)
        }
      }
    }

    async fn say_hello(
      &self,
      request: Request<HelloUserRequest>,
    ) -> Result<Response<HelloUserResponse>, Status> {
      let req_body = request.into_inner();
      let res_body = HelloUserResponse {
          hello_output: format!("Welcome, {}!", req_body.username.to_string()),
      };

      Ok(Response::new(res_body))
    }
}
