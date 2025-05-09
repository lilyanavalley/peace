
// #![recursion_limit = "256"]
#![warn(missing_docs)]

#[cfg(feature = "ssr")]
pub mod server;

#[cfg(any(feature = "ssr", feature = "hydrate"))]
use peace::{
  app::*,
  config,
  placeholders
};

// #[cfg(any(feature = "ssr", feature = "hydrate"))]

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  use std::sync::Mutex;
  // extern crate jsonwebtoken;
  use actix_files::Files;
  use actix_web:: { * , dev::*, body::*, web::Data };
  use leptos::{ prelude::*, config::get_configuration, logging::* };
  use leptos_meta::MetaTags;
  use leptos_actix::{generate_route_list, LeptosRoutes};
  use mongodb;
  // use webauthn_rs;
  use chrono;
  // use actix_web_grants::{ protect, authorities::AttachAuthorities };
  // use actix_web_httpauth::{ middleware::HttpAuthentication, extractors::bearer::BearerAuth };
  use serde::{Deserialize, Serialize};


  log!("leptos configurating...");
  let leptos_config = get_configuration(Some("Cargo.toml")).unwrap();
  let leptos_address = leptos_config.leptos_options.site_addr;

  log!("peace configurating...");
  let mut peace_config = config::PeaceConfig::prime_envs();
  log!("mongodb client setup...");
  let db_mongodb = peace_config.mongodb_setup();

  log!("priming quote of the day...");
  peace_config.prime_qotd(&db_mongodb).await.unwrap();
  log!("...ready!");
  let peace_config = Data::new(Mutex::new(peace_config));
  
  log!("Starting server...");
  HttpServer::new(move || {
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    let leptos_options = &leptos_config.leptos_options;
    let site_root = leptos_options.site_root.clone().to_string();
    // let authenticator = HttpAuthentication::bearer(server::auth_validate);
    // println!("listening on http://{}", &addr);

    log!("...app launch initiated.");
    App::new()
      // serve JS/WASM/CSS from `pkg`
      .service(Files::new("/pkg", format!("{site_root}/pkg")))
      // serve other assets from the `assets` directory
      .service(Files::new("/assets", &site_root))
      // serve the favicon from /favicon.ico
      .service(favicon)
      .leptos_routes(routes, {
        let leptos_options = leptos_options.clone();
        move || {
          view! {
            <!DOCTYPE html>
            <html lang="en">
              <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=leptos_options.clone() />
                <HydrationScripts options=leptos_options.clone()/>
                <MetaTags/>
              </head>
              <body>
                <App/>
              </body>
            </html>
          }
        }
      })
      // .service(
      //   web::scope("authenticate/")
      //     .wrap(authenticator)
      // )
      .app_data(web::Data::new(leptos_options.to_owned()))
      .app_data(web::Data::new(db_mongodb.clone()))
      .app_data(web::Data::clone(&peace_config))
      // .wrap(middleware::from_fn(server::rate_limit))
  })
  .bind(&leptos_address)?
  .run()
  .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
  leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
  let leptos_options = leptos_options.into_inner();
  let site_root = &leptos_options.site_root;
  Ok(actix_files::NamedFile::open(format!(
    "{site_root}/favicon.ico"
  ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
  // no client-side main function
  // unless we want this to work with e.g., Trunk for pure client-side testing
  // see lib.rs for hydration function instead
  // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
  // a client-side main function is required for using `trunk serve`
  // prefer using `cargo leptos serve` instead
  // to run: `trunk serve --open --features csr`
  use peace::app::*;

  console_error_panic_hook::set_once();

  leptos::mount_to_body(App);
}
