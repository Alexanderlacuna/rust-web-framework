
pub mod schema;
pub mod models;
pub mod users;
#[macro_use]
extern crate  diesel;
extern crate  dotenv;


use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use self::models::{User,NewUser};

use users::create_users;

pub fn create_user<'a>(conn:&SqliteConnection,username:&'a str,email:&'a str){
    use schema::users;
    let new_user=NewUser{
        username,
        email

    };

    diesel::insert_into(users::table).values(&new_user)
    .execute(conn).expect("Error while trying to save");
}

pub fn establish_connection()->SqliteConnection{
    dotenv().ok();
    let database_url=env::var("DATABASE_URL")
    .expect("Database url must be set");
    SqliteConnection::establish(&database_url)
    .expect("Error while trying to connect")
}