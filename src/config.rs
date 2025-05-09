
use leptos::{ prelude::*, logging::* };
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use mongodb;


const PEACE_WEBAUTHN_ID:      &'static str = "PEACE_WEBAUTHN_ID";
const PEACE_WEBAUTHN_ORIGIN:  &'static str = "PEACE_WEBAUTHN_ORIGIN";
const PEACE_WEBAUTHN_NAME:    &'static str = "PEACE_WEBAUTHN_NAME";
const PEACE_WEBAUTHN_TIMEOUT: &'static str = "PEACE_WEBAUTHN_TIMEOUT";
const PEACE_MONGODB_URIS:     &'static str = "PEACE_MONGODB_URIS";
const PEACE_MONGODB_USER:     &'static str = "PEACE_MONGODB_USER";
const PEACE_MONGODB_PASSWORD: &'static str = "PEACE_MONGODB_PASSWORD";
// const PEACE_MONGODB_TLS:      &'static str = "PEACE_MONGODB_TLS";
const RAW_CA_CERT:            &'static str = "MONGODB_CA_CERT";

const FILEPATH_CA:            &'static str = "ca.crt";

#[derive(Clone)]
pub struct PeaceConfig {
  pub webauthn_rp_id:     String,
  pub webauthn_rp_origin: String,
  pub webauthn_rp_name:   Option<String>,
  pub webauthn_timeout:   std::time::Duration,
  pub quotes:             quote_queue::QuoteDocument,
  #[cfg(feature = "ssr")]
  mongodb_uris:           Vec<mongodb::options::ServerAddress>,
  #[cfg(feature = "ssr")]
  mongodb_tls:            Option<mongodb::options::TlsOptions>,
  #[cfg(feature = "ssr")]
  mongodb_user:           Option<String>,
  #[cfg(feature = "ssr")]
  mongodb_password:       Option<String>,
}

impl PeaceConfig {

  #[cfg(feature = "ssr")]
  pub fn prime_envs() -> Self {
    use std::str::FromStr;

    log!("reading environment:");
    let mut config = PeaceConfig::default();
    let envs = std::env::vars();

    for (each_key, each_value) in envs {
      match each_key.as_ref() {
        // TODO: Document all of these.
        PEACE_WEBAUTHN_ID       => config.webauthn_rp_id = each_value,
        PEACE_WEBAUTHN_ORIGIN   => config.webauthn_rp_origin = each_value,
        PEACE_WEBAUTHN_NAME     => config.webauthn_rp_name = Some(each_value),
        PEACE_WEBAUTHN_TIMEOUT  => config.webauthn_timeout = std::time::Duration::from_secs(each_value.parse::<u64>().unwrap()),
        PEACE_MONGODB_URIS      => config.mongodb_uris = Self::parse_mongodb_uris(each_value),
        PEACE_MONGODB_USER      => config.mongodb_user = Some(each_value),
        PEACE_MONGODB_PASSWORD  => config.mongodb_password = Some(each_value),
        // PEACE_MONGODB_TLS       => config.mongodb_tls = Self::parse_mongodb_tls(each_value),
        RAW_CA_CERT             => {
          log!("  - {each_key} defined");
          Self::make_cert_file(each_value).unwrap();
          config.mongodb_tls = Some(mongodb::options::TlsOptions::builder()
            .allow_invalid_certificates(false)
            .ca_file_path(Some(std::path::PathBuf::from_str(FILEPATH_CA).unwrap()))
            .build())
        },
        _ => log!("  x {each_key}"),
      }
    }

    config

  }

  #[cfg(feature = "ssr")]
  fn parse_mongodb_uris(uris: String) -> Vec<mongodb::options::ServerAddress> {
    log!("unparsed uris: {uris}");
    // TODO: more complicated logic may be implemented to prevent downstream errors from occuring due to bad input.
    let mut returnable = Vec::new();
    for each_uri in uris.split(',') {
      if let Ok(each_uri) = mongodb::options::ServerAddress::parse(each_uri) {
        log!("  - {each_uri}");
        returnable.push(each_uri);
      } else { leptos::logging::warn!("env {PEACE_MONGODB_URIS} specifies invalid uri: {each_uri}"); }
    }
    returnable
  }

  #[cfg(feature = "ssr")]
  fn make_cert_file(cert: String) -> std::io::Result<()> {
    use std::io::Write;

    log!("making cert file at {FILEPATH_CA}");
    let mut buffer = cert.as_bytes();
    let mut file = std::fs::File::create(FILEPATH_CA)?;
    file.write_all(&mut buffer)?;

    Ok(())
  }

  #[cfg(feature = "ssr")]
  pub async fn prime_qotd(&mut self, mongodb: &mongodb::Client) -> mongodb::error::Result<()> {
    log!("await point");
    self.quotes = quote_server::fetch(mongodb).await?;
    Ok(())
  }

  #[cfg(feature = "ssr")]
  pub fn get_mongodb_uris(&self) -> Vec<mongodb::options::ServerAddress> {
    self.mongodb_uris.clone()
  }

  #[cfg(feature = "ssr")]
  pub fn mongodb_setup(&mut self) -> mongodb::Client {
    
    let mut credential = None;
    if self.mongodb_password.is_some() || self.mongodb_user.is_some() {
      log!("credentials are being set for mongodb client...");
      let username = self.mongodb_user.clone();
      let password = self.mongodb_password.clone();
      credential = Some(
        mongodb::options::Credential::builder()
          .username(username)
          .password(password)
          .build()
      )
    }

    let mut tls = None;
    if self.mongodb_tls.is_some() {
      log!("tls is being enabled for mongodb client...");
      let tls_options = self.mongodb_tls.clone().unwrap();
      tls = Some(mongodb::options::Tls::Enabled(tls_options))
    }

    if self.mongodb_uris.is_empty() {
      warn!("there are NO mongodb addresses to connect to!");
      log!("fallback to address: mongodb://127.0.0.1:27017");
      self.mongodb_uris.push(mongodb::options::ServerAddress::Tcp { host: "127.0.0.1".to_string(), port: Some(27017) });
    }

    log!("building client...");
    mongodb::Client::with_options(
      mongodb::options::ClientOptions::builder()
        .hosts(self.mongodb_uris.clone())
        .tls(tls)
        .credential(credential)
        .app_name(Some(env!("CARGO_PKG_NAME").to_string()))
        .build()
    ).unwrap()

  }

}

#[cfg(feature = "ssr")]
impl Default for PeaceConfig {
  fn default() -> Self {
    PeaceConfig {
      webauthn_rp_id:     "127.0.0.1".to_string(),
      webauthn_rp_origin: "127.0.0.1:3000".to_string(),
      webauthn_rp_name:   None,
      webauthn_timeout:   webauthn_rs::DEFAULT_AUTHENTICATOR_TIMEOUT,
      mongodb_uris:       Vec::new(),
      quotes:             quote_queue::QuoteDocument::default(),
      mongodb_tls:        None,
      mongodb_user:       None,
      mongodb_password:   None,
    }
  }
}

#[cfg(any(feature = "ssr", feature = "hydrate"))]
mod quote_queue {

  use crate::components::favoritequotes::ReturnedQuote;
  use leptos::logging::log;


  #[derive(Default, Clone)]
  pub struct QuoteDocument {
    pub quote:  Option<ReturnedQuote>, // * The quote to be displayed.
    pub expire: Option<u64>, // * Expiration time of quote queue in unix epoch.
  }

  impl QuoteDocument {
    
    pub fn returned_quote(&self) -> Option<ReturnedQuote> {
      self.quote.clone()
    }

    #[cfg(feature = "ssr")]
    pub async fn queued_or_fetching(&mut self, mongodb: &mongodb::Client) -> mongodb::error::Result<()> {
      use super::quote_server::fetch;
      if self.expires() && self.expired() {
        log!("QuoteDocument::queued_or_fetching - expired, fetching new quote");
        let quote = fetch(mongodb).await?;
        self.quote = quote.quote;
        self.expire = quote.expire;
      }
      Ok(())
    }
  
    pub fn expired(&self) -> bool {
      if let Some(future_time) = self.expire {
        let expires = future_time.lt(
          &std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
          );
          expires
      } else {
        false
      }
    }
  
    pub fn expires(&self) -> bool {
      self.expire.is_some()
    }

  }


}

#[cfg(feature = "ssr")]
mod quote_server {
  
  use mongodb;
  use leptos::logging::log;
  use crate::components::favoritequotes::ReturnedQuote;
  use super::quote_queue::QuoteDocument;

  const PROFILE_DB_COLLECTION_QUOTES: &'static str = "quotes";
  const PROFILE_DB_DATABASE: &'static str = "profile";

    
  pub async fn fetch(mongodb: &mongodb::Client) -> mongodb::error::Result<QuoteDocument> {

    log!("reading quote collection");
    let collection = mongodb
      .database(PROFILE_DB_DATABASE)
      .collection::<ReturnedQuote>(PROFILE_DB_COLLECTION_QUOTES)
      .aggregate([mongodb::bson::doc! { "$sample": { "size": 1 } }])
      .with_type::<ReturnedQuote>()
      .await;

    let collection = collection?;

    log!("deserializing quote...");
    let quote = collection.deserialize_current().ok();
    
    // Set expiration time for a future Unix Epoch timestamp, returning `None` if bounds of u64 overflow.
    let expire = Some(
      std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .wrapping_add(86400) // 24 hours
    );

    log!("quote enqueued!");
    Ok(QuoteDocument {
      quote,
      expire
    })

  }

}

#[cfg(test)]
mod tests {

  use super::*;
  use std::{ env, time::Duration };
  use mongodb::options::ServerAddress;


  #[test]
  fn env_priming() {

    // * Test all values set non-default.
    let webauthn_id = "test";
    let webauthn_origin = "test.example.com:8080";
    let webauthn_name = "Testing";
    let webauthn_timeout = "60";
    
    #[cfg(not(target_family = "unix"))]
    let mongodb_uris = "localhost:1000,127.0.0.1:1001";
    #[cfg(not(target_family = "unix"))]
    let mongodb_uris_hardtruth = vec![
      ServerAddress::Tcp { host: "localhost".to_string(), port: Some(1000) },
      ServerAddress::Tcp { host: "127.0.0.1".to_string(), port: Some(1001) },
    ];

    #[cfg(target_family = "unix")]
    let mongodb_uris = "localhost:1000,127.0.0.1:1001,unix://socket.sock";
    #[cfg(target_family = "unix")]
    let mongodb_uris_hardtruth = vec![
      ServerAddress::Tcp { host: "localhost".to_string(), port: Some(1000) },
      ServerAddress::Tcp { host: "127.0.0.1".to_string(), port: Some(1001) },
      ServerAddress::Unix { path: std::path::PathBuf::from("unix://socket.sock") },
    ];

    // * Vars are (temporarily) set for detection for `prime_envs()`
    unsafe {
      env::set_var(PEACE_WEBAUTHN_ID, webauthn_id);
      env::set_var(PEACE_WEBAUTHN_ORIGIN, webauthn_origin);
      env::set_var(PEACE_WEBAUTHN_NAME, webauthn_name);
      env::set_var(PEACE_WEBAUTHN_TIMEOUT, webauthn_timeout);
      env::set_var(PEACE_MONGODB_URIS, mongodb_uris);
    }

    // * PeaceConfig should pull the vars above and populate itself with those parsed values.
    let config = PeaceConfig::prime_envs();

    assert_eq!(config.webauthn_rp_id.as_str(), webauthn_id);
    assert_eq!(config.webauthn_rp_origin.as_str(), webauthn_origin);
    assert_eq!(config.webauthn_rp_name, Some(webauthn_name.to_string()));
    assert_eq!(config.webauthn_timeout, Duration::from_secs(webauthn_timeout.parse::<u64>().unwrap()));
    assert_eq!(config.mongodb_uris, mongodb_uris_hardtruth);

  }

  #[test]
  fn defaults() {

    let config = PeaceConfig::default();
    let config_wa_rp_or: std::net::SocketAddr = config.webauthn_rp_origin.parse().unwrap();

    assert!(config.webauthn_rp_name.is_none()); // No 'friendly name' is set by default.
    assert!(config_wa_rp_or.ip().is_loopback()); // Origin address is localhost.
    assert_eq!(config_wa_rp_or.port(), 3000); // Port set to application server default.
    assert_eq!(config.webauthn_rp_id, "127.0.0.1".to_string()); // Address set to localhost.
    assert_eq!(config.webauthn_timeout, std::time::Duration::from_secs(300)); // Webauthn timeout default is 300s.

    assert!(config.mongodb_uris.is_empty()); // No mongodb servers specified by default.

    assert!(config.mongodb_user.is_none());
    assert!(config.mongodb_password.is_none());
    assert!(config.mongodb_tls.is_none());

  }

}
