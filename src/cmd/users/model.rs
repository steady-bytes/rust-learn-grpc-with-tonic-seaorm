/////////////
/// MODEL ///
/////////////
use diesel::pg::{PgConnection};
use diesel::prelude::*;
use validator::{Validate};
use crate::dao::schema::users;

//////////////////////////////////////////////////////////////
/// ENTITY - An uniqly identifiable row/value in the database.
//////////////////////////////////////////////////////////////

/// User is the model representing a user `Entity` type of the system.
/// The `id` is the primary key, and generally how a `User` is identified, 
/// or found in in the system.
#[derive(Debug, PartialEq, Clone, Queryable, Insertable, Validate)]
#[table_name="users"]
pub struct User {
  /// TODO -> figure out validation for uuid
  pub id: uuid::Uuid,
  #[validate(length(min = 1, max = 255))]
  pub username: String,
  #[validate(length(min = 1, max = 255))]
  pub first_name: String,
  #[validate(length(min = 1, max = 255))]
  pub last_name: String,
  #[validate(email)]
  pub email: String,
  #[validate(length(min = 16))]
  pub password: String,
}

////////////////////////////////////////////////////////////////////////////////////
/// FORM - Is a struct type that represents input values to a model. Each key is
/// optional so that updates can be derived from a `Diesel` marco. The only value
/// that is not optional should be the id, or primary key.
////////////////////////////////////////////////////////////////////////////////////

/// UserForm - Is the `Form` type for the `User` model.
#[derive(Debug, PartialEq, Clone, AsChangeset, Identifiable, Insertable, Queryable, Validate)]
#[table_name="users"]
pub struct UserForm { 
  /// TODO -> figure out validation for uuid
  pub id: uuid::Uuid,
  #[validate(length(min = 1, max = 255))]
  pub username: Option<String>,
  #[validate(length(min = 1, max = 255))]
  pub first_name: Option<String>,
  #[validate(length(min = 1, max = 255))]
  pub last_name: Option<String>,
  #[validate(email)]
  pub email: Option<String>,
  #[validate(length(min = 16))]
  pub password: Option<String>,
}

// TODO -> change to error codes with a message
/// ERRORS
pub type UserFormFromUpdateFieldsError = String;

/// RESULTS
pub type UserFormFromUpdateFieldsResult = Result<UserForm, UserFormFromUpdateFieldsError>;

/// CREATE, READ, UPDATE, DELETE
impl UserForm {
  pub fn new(id: uuid::Uuid) -> UserForm { 
    UserForm{
      id: id,
      username: None,
      first_name: None,
      last_name: None,
      email: None,
      password: None,
    }
  }

  pub fn create(self, db_conn: &PgConnection) -> QueryResult<User> {
    diesel::insert_into(users::table) 
      .values(self)
      .get_result(db_conn)
  }

  pub fn read(self, db_conn: &PgConnection) -> QueryResult<User> {
    use crate::dao::schema::users::dsl::*;
    users.filter(id.eq(self.id)).first::<User>(&*db_conn)
  }

  pub fn update(self, db_conn: &PgConnection) -> QueryResult<User> {
    self.save_changes(db_conn)
  }

  pub fn delete(self, db_conn: &PgConnection) -> QueryResult<User> {
    use crate::dao::schema::users::dsl::*;
    diesel::delete(users.filter(id.eq(self.id))).get_result(db_conn)
  }

  /// This is used b/c protobuf's don't support optionals and have opinions of default values for attributes
  /// that are not set. For example if a string was not set, in protobuf's they use a string that is of length 0.
  /// meaning that an update may be mistaken for an accidental change.
  pub fn from_user_and_update_fields(u: User, fields: Vec<String>) -> UserFormFromUpdateFieldsResult {
    // check id
    if u.id.is_nil() {
      return Err("id is empty".to_string()) 
    }

    // create a UserFrom struct
    let mut user_form = UserForm::new(u.id);
    user_form.id = u.id;

    // iterate through fields, and set in the UserForm
    for f in fields.iter() {
      match f.as_ref() {
        "username" => user_form.username = Some(u.username.clone()),
        "first_name" => user_form.first_name = Some(u.first_name.clone()),
        "last_name" => user_form.last_name = Some(u.last_name.clone()),
        "email" => user_form.email = Some(u.email.clone()),
        // TODO -> this should return an error
        _ => continue
      }
    }

    Ok(user_form)
  }
}

/// READ A LIST [Unused]

/// UserList is a `NewType` following the new type idiom commonly found in rust. It's an abstration
/// so that re-exports are not needed on when integrating the `UserPB` type and the `User` model type.
/// Currenty this level of integration is needed b/c we can't derive the same behavior on the `UserPB`
/// type since it's generated outside of this codebase. A differnt possiable solution would be change
/// the compiler that is used to create the rust types
#[derive(Clone)]
pub struct UsersList(pub Vec<User>);

impl UsersList {
  pub fn find_all(self, db_conn: &PgConnection) -> QueryResult<Vec<User>> {
    use crate::dao::schema::users::dsl::*;

    let results: Result<Vec<User>, diesel::result::Error> = users.load::<User>(&*db_conn);
    match results {
      Ok(r) => Ok(r),
      Err(e) => Err(e)
    }
  }
}
