#![recursion_limit = "512"]
#[macro_use]
extern crate diesel;
extern crate bigdecimal;
extern crate chrono;
extern crate dotenvy;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

mod models;
mod schema;

use models::JobstatJob;
use schema::jobstat_jobs::dsl::*;
use crate::schema::jobstat_jobs;

pub fn establish_connection(database_url: &str) -> PgConnection {
    dotenv().ok();
    let database_url = env::var(database_url).expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let mut connection_in = establish_connection("DATABASE_IN_URL");
    let mut connection_out = establish_connection("DATABASE_OUT_URL");
    let results = jobstat_jobs
        .limit(8)
        .load::<JobstatJob>(&mut connection_in)
        .expect("Error loading members");

    diesel::sql_query("TRUNCATE jobstat_jobs")
        .execute(&mut connection_out)
        .expect("Error truncating the table");

    diesel::insert_into(jobstat_jobs::table)
        .values(&results)
        .execute(&mut connection_out)
        .expect("Error saving to new database");

    for member in results {
        println!("{:?} {:?}", member.id, member.login);
    }
}
