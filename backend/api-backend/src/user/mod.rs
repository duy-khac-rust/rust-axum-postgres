// region:      --- Modules

mod routes;
mod services;


// -- Flatten

pub use domain_backend::*;
pub use inftra_backend::base::*;
pub use routes::*;


// endregion:   --- Modules

pub struct UserDmc;

impl DMC for UserDmc {
    const SCHEMA: &'static str = "user";
    const TABLE: &'static str = "tbl_user";
    const PRIMARY_KEY_ID: &'static str = "pk_user_id";
    
}
