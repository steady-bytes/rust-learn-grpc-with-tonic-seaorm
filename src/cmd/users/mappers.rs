use crate::api::user_management::users_v1::{User as UserPB};
use crate::cmd::users::model::{User, UsersList, UserForm};

/////////////////////////////////////////////////////////////////
/// FROM impls - Translators                                  ///
/// Used as tranlators between our protobuf type's and models ///
/////////////////////////////////////////////////////////////////

/// Convert a `&User` to a `UserPB`
impl From<&User> for UserPB {
  fn from(m: &User) -> Self {
  
    UserPB {
      id: m.id.to_string(), 
      username: m.username.clone(),
      first_name: m.first_name.clone(),
      last_name: m.last_name.clone(),
      email: m.email.clone(),
      password: m.password.clone(),
    }
  }
}

/// Convert a `User` to a `UserPB`
impl From<User> for UserPB {
  fn from(m: User) -> Self {
  
    UserPB {
      id: m.id.to_string(), 
      username: m.username.clone(),
      first_name: m.first_name.clone(),
      last_name: m.last_name.clone(),
      email: m.email.clone(),
      password: m.password.clone(),
    }
  }
}

/// Convert a `&User` to a `User`
impl From<&User> for User {
  fn from(m: &User) -> User {
    User {
      id: m.id,
      username: m.username.clone(),
      first_name: m.first_name.clone(),
      last_name: m.last_name.clone(),
      email: m.email.clone(),
      password: m.password.clone(),
    }
  }
}

/// Convert a `UserPB` to a `User
impl From<UserPB> for User {
  fn from(pb: UserPB) -> User {

    User {
      id: uuid::Uuid::parse_str(&pb.id).unwrap(),
      username: pb.username,
      first_name: pb.first_name,
      last_name: pb.last_name,
      email: pb.email,
      password: pb.password,
    }
  }
}

/// Convert a `UserList` to a `Vec<UserPB>`
impl From<UsersList> for Vec<UserPB> {
  fn from(ul: UsersList) -> Self {
    let mut users_list_pb: Vec<UserPB> = Vec::new();

    for u in ul.0.iter() {
      users_list_pb.push(u.into());
    }

    users_list_pb
  }
}

/// Convert `UsersList` to a `Vec<User>`
impl From<UsersList> for Vec<User> {
  fn from(ul: UsersList) -> Self {
    let mut users_list: Vec<User> = Vec::new();

    for u in ul.0.iter() {
      users_list.push(u.into());
    }

    users_list
  }
}

/// Convert a `UserPB` to a `UserForm`
impl From<UserPB> for UserForm {
  fn from(u: UserPB) -> Self { 

    UserForm {
      id: uuid::Uuid::new_v4(),
      username: Some(u.username),
      first_name: Some(u.first_name),
      last_name: Some(u.last_name),
      email: Some(u.email),
      password: Some(u.password),
    }
  }
}
