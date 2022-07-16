use async_trait::async_trait;
use entity::user;
use crate::cmd::{SingularCrud};
use sea_orm::{DatabaseConnection, IntoActiveModel, DbErr, ActiveModelTrait, EntityTrait, ActiveValue::Set};
use futures::executor::block_on;

////////////////////////
/// USERS CONTROLLER ///
////////////////////////

/// UsersController - Responsible to implement all of the business logic worhflows
/// of the user entity.
pub struct UsersController {
  db_connection: DatabaseConnection
}

impl UsersController {
  pub fn default(db_address: String) -> Option<Self> {
    let db_conn_future = sea_orm::Database::connect(&db_address);

    // not using await b/c this is not an async function
    match block_on(db_conn_future) {
      Ok(db) => {
        Some(UsersController {
          db_connection: db,
        })
      },
      Err(_e) => None
    }
  }
}

#[async_trait]
impl SingularCrud for UsersController {
  type Entity = user::ActiveModel;
  type InputFormFields = user::ActiveModel;

  async fn create(&self, m: Self::InputFormFields) -> Result<Self::Entity, DbErr> { 
    let create = m.insert(&self.db_connection.clone()).await;

    match create {
      Ok(v) => Ok(v.into_active_model()),
      Err(_) => Err(DbErr::Exec(String::from("new user entity was not saved"))),
    }
  }

  async fn read(&self, m: Self::InputFormFields) -> Result<Self::Entity, DbErr> {
    let read = user::Entity::find_by_id(m.id.unwrap()).one(&self.db_connection.clone()).await?;

    match read {
      Some(v) => Ok(v.into_active_model()),
      None => Err(DbErr::Exec(String::from("entity was not found by provided id")))
    }
  }

  async fn update(&self, m: Self::InputFormFields) -> Result<Self::Entity, DbErr> {
    // handle the option in a better way, for example if the entity can't be found then a better error needs to be returned
    let mut update_user: user::ActiveModel = user::Entity::find_by_id(m.id.unwrap())
      .one(&self.db_connection.clone())
      .await
      .unwrap()
      .unwrap()
      .into();

    // TODO -> find a better way to map requests to attributes
    update_user.last_name = Set(m.last_name.unwrap());

    let update = update_user.update(&self.db_connection.clone()).await;

    match update {
      Ok(v) => Ok(v.into_active_model()),
      Err(_) => Err(DbErr::Exec(String::from("failed to update user entityt"))),
    }
  }

  async fn delete(&self, m: Self::InputFormFields) -> Result<bool, DbErr> {
    // TODO -> handle the option better.
    let delete_user: user::ActiveModel = user::Entity::find_by_id(m.id.unwrap())
      .one(&self.db_connection.clone())
      .await
      .unwrap()
      .unwrap()
      .into();
  
    let status = delete_user.delete(&self.db_connection.clone()).await;

    match status {
      Ok(_) => Ok(true),
      Err(_) => Err(DbErr::Exec(String::from("failed to delete user entity"))),
    }
  }
}
