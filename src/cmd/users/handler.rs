use log::{debug, info};
use std::str::{FromStr};
use sea_orm::prelude::Uuid;
use sea_orm::ActiveValue::NotSet;
use tonic::{Request, Response, Status};
use crate::api::user_management::users_v1::User;
use crate::cmd::SingularCrud;
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

use sea_orm::{self, entity::*};
use entity::user;

///////////////////////////////////
/// ********* HANDLER ********* ///
///////////////////////////////////

pub struct UsersHandler {
  service: UsersController,
}

impl UsersHandler {
  pub fn default(service: UsersController) -> Self {
    UsersHandler {
      service: service,
    }
  }
}

#[tonic::async_trait]
impl Users for UsersHandler {
    async fn signup(
        &self,
        request: Request<SignupUserRequest>,
    ) -> Result<Response<SignupUserResponse>, Status> {
        info!("signup request: {:?}", request);

        // extract request values into `UserForm`
        let input = request.into_inner().model.unwrap();

        let insert_details = user::ActiveModel {
          id: Set(Uuid::new_v4()),
          username: Set(input.username.clone()),
          first_name: Set(input.first_name.clone()),
          last_name: Set(input.last_name.clone()),
          email: Set(input.email.clone()),
          password: Set(input.password.clone()), 
        };

        // validate 
      
        // todo -> policy enforcement

        let res = SignupUserResponse{
          id: self.service.create(insert_details).await.unwrap().id.unwrap().to_string(),
        };
    
        Ok(Response::new(res))
    }

    async fn get_by_id(
      &self,
      request: Request<GetUserByIdRequest>,
    ) -> Result<Response<GetUserByIdResponse>, Status> {
      info!("get_by_id request: {:?}", request);

      // extract request values into `UserForm`
      let input = request.into_inner().id;

      // validate 

      // todo -> policy enforcement

      let read_details = user::ActiveModel {
        id: Set(Uuid::from_str(&input).unwrap()),
        username: NotSet,
        first_name: NotSet,
        last_name: NotSet,
        email: NotSet,
        password: NotSet,
      };

      // call the service layer to execute the business logic for this request
      let found = self.service.read(read_details).await;

      match found {
        Ok(v) => {
          let res = GetUserByIdResponse{
            model:  Some(User{
              id: v.id.unwrap().to_string(),
              username: v.username.unwrap(),
              first_name: v.first_name.unwrap(),
              last_name: v.last_name.unwrap(),
              email: v.email.unwrap(),
              // in a read example def don't return this 
              password: v.password.unwrap(),
            })
          };
          Ok(Response::new(res))
        },
        Err(_) => Err(Status::new(tonic::Code::Internal, "failed to find users by id".to_owned()))
      } 
    }

    // TODO: This still needs to be done, but for now I have all I need for an example
    async fn update_by_id(
      &self,
      request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserResponse>, Status> {
      debug!("update_by_id: {:?}", request);

      // extract reqquest values into `User` 

      // validate
      
      // todo -> policy enforcement

      // create `UserForm` to update values in the database
      Err(Status::new(tonic::Code::Internal, "not implemented".to_owned()))
    }

    // TODO: This still needs to be done, but for now I have all I need for an example
    async fn delete_by_id(
      &self,
      request: Request<DeleteByIdRequest>,
    ) -> Result<Response<DeleteByIdResponse>, Status> {
      debug!("delete_by_id request: {:?}", request);
      
      // extract request values into `UserForm`

      // todo -> Policy Enforement

      Err(Status::new(tonic::Code::Internal, "not implemented".to_owned()))
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
