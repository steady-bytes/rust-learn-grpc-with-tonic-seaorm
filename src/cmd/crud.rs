pub trait SingularCrud {
  /// Entity is the representation of what table, and it's schema. Crud operations
  /// are implemented for this type, and it's the return value for each method.
  type Entity;

  /// Often the input values to an entity are not the same at the returned model.
  /// providing the `InputInputFormFields` allows for this flexibility.
  type InputFormFields;

  /// Error, this is generally an enum of codes
  type Error;

  /// Create one `Entity` given a set of `InputInputFormFields`.
  fn create(&self, ff: Self::InputFormFields) -> Result<Self::Entity, Self::Error>;

  /// Read one `Entity` given a set of `InputInputFormFields` that contains the uuid
  /// of the desired `Entity`
  fn read(&self, ff: Self::InputFormFields) -> Result<Self::Entity, Self::Error>;

  /// Update one `Entity` with the new values in the `InputFormFields` it's recommended
  /// to use `Options` for the key/values in `InputFormFields` so that on the non-None
  /// values are changed in the `Entity`
  fn update(&self, ff: Self::InputFormFields) -> Result<Self::Entity, Self::Error>;

  /// Delete one `Entity` if found using the `InputFormFields` id.
  fn delete(&self, ff: Self::InputFormFields) -> Result<Self::Entity, Self::Error>;
}

/// NOTE: Can I maybe turn this into a macro, that could be expanded?
pub trait Creator {
  type Entity;
  type InputFormFields;
  type Error;

  fn create_one(&self, ff: Self::InputFormFields) -> Result<Self::Entity, Self::Error>;
}

pub trait Reader {
  type Entity;
  type InputFormFields;
  type Error;

  fn read_one(&self, ff: Self::InputFormFields) -> Result<Self::Entity, Self::Error>;
}

pub trait Updater {
  type Entity;
  type InputFormFields;
  type Error;

  fn update_one(&self, ff: Self::InputFormFields) -> Result<Self::Entity, Self::Error>; 
}

pub trait Deleter {
  type Entity;
  type InputFormFields;
  type Error;

  fn delete_one(&self, ff: Self::InputFormFields) -> Result<Self::Entity, Self::Error>; 
}
