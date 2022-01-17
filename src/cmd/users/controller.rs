use diesel::r2d2::{Pool, ConnectionManager};
use diesel::pg::{PgConnection};

use crate::cmd::users::model;
use crate::cmd::crud::{SingularCrud};

////////////////////////
/// USERS CONTROLLER ///
////////////////////////

/// UsersController - Responsible to implement all of the business logic worhflows
/// of the user entity.
pub struct UsersController {
  db_connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl UsersController {
  pub fn default(db_address: String) -> Self {
    let manager = ConnectionManager::<PgConnection>::new(db_address);
    let pool = r2d2::Pool::new(manager).unwrap();

    UsersController {
      db_connection_pool: pool,
    }
  }
}

impl SingularCrud for UsersController {
  type Entity = model::User;
  type InputFormFields = model::UserForm;
  type Error = String;

  fn create(&self, m: Self::InputFormFields) -> Result<model::User, Self::Error> { 
    let conn = self.db_connection_pool.get().unwrap();
    let user = m.create(&*conn).unwrap();

    // todo -> create a user in the OAuth provider

    Ok(user)
  }

  fn read(&self, m: Self::InputFormFields) -> Result<model::User, Self::Error> {
    let conn = self.db_connection_pool.get().unwrap();
    let user = m.read(&*conn).unwrap();

    Ok(user)
  }

  fn update(&self, m: Self::InputFormFields) -> Result<model::User, Self::Error> {
    let conn = self.db_connection_pool.get().unwrap();
    let user = m.update(&*conn).unwrap(); 
    
    Ok(user)
  }

  fn delete(&self, m: Self::InputFormFields) -> Result<model::User, Self::Error> {
    let conn = self.db_connection_pool.get().unwrap();
    let user = m.delete(&*conn).unwrap();

    Ok(user)
  }
}
