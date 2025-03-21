
use mongodb;
use leptos::*;
use leptos::prelude::*;
use actix_web::web;
use std::sync::Arc;

pub const MONGODB_D_PROFILE:  &'static str = "profile";
pub const MONGODB_C_PROFILE_QUOTES: &'static str = "quotes";


// Retrieve database from the connected MongoDB server.
pub async fn mongodb_database(
  mongodb: web::Data<mongodb::Client>, 
) -> Result<mongodb::Database, ServerFnError> {

  Ok(mongodb.database("test"))

}
