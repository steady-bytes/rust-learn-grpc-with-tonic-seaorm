use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_users_management_domain().unwrap();

    Ok(())
}

fn build_users_management_domain() -> Result<(), Box<dyn std::error::Error>> { 
  let out_dir = Path::new("./src/api/user_management");
  let includes = Path::new("mod.rs");

  // configure the tonic builder
  tonic_build::configure()
      .out_dir(out_dir)
      .include_file(includes)
      .compile(&[
        "./protos/user_management/users.proto",
      ], &["."])?;

  Ok(())
}