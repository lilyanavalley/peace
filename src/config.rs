
use leptos::{ prelude::*, logging::* };
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use mongodb;
#[cfg(feature = "ssr")]
use std::future::Future;


const PEACE_WEBAUTHN_ID:      &'static str = "PEACE_WEBAUTHN_ID";
const PEACE_WEBAUTHN_ORIGIN:  &'static str = "PEACE_WEBAUTHN_ORIGIN";
const PEACE_WEBAUTHN_NAME:    &'static str = "PEACE_WEBAUTHN_NAME";
const PEACE_WEBAUTHN_TIMEOUT: &'static str = "PEACE_WEBAUTHN_TIMEOUT";
const PEACE_MONGODB_URIS:     &'static str = "PEACE_MONGODB_URIS";
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
  mongodb_uris:           Option<String>,
  #[cfg(feature = "ssr")]
  mongodb_tls:            Option<mongodb::options::Tls>,
  #[cfg(feature = "ssr")]
  pub mongodb_client:     Option<mongodb::Client>,
  #[cfg(feature = "ssr")]
  documents:              document_queue::Documents,
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
        PEACE_MONGODB_URIS      => config.mongodb_uris = Some(each_value),
        RAW_CA_CERT             => {
          log!("  🔑 {each_key} defines a TLS setup");
          log!("  🔑   → dumping CA cert to disk");
          Self::make_cert_file(each_value).unwrap();
          config.mongodb_tls = Some(
            mongodb::options::Tls::from(
              mongodb::options::TlsOptions::builder()
                .allow_invalid_certificates(false)
                .ca_file_path(Some(std::path::PathBuf::from_str(FILEPATH_CA).unwrap()))
                .build())
          )
        },
        _ => log!("  〰️ {each_key}"),
      }
    }

    config

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
  pub async fn prime_qotd(&mut self) -> mongodb::error::Result<()> {
    if self.mongodb_client.is_none() {
      warn!("cannot prime quote of the day without a mongodb client!");
      return Ok(());
    }
    self.quotes = quote_server::fetch(self.mongodb_client.clone().unwrap().as_borrowed()).await?;
    Ok(())
  }

  // #[cfg(feature = "ssr")]
  // pub fn get_mongodb_uris(&self) -> Vec<mongodb::options::ServerAddress> {
  //   self.mongodb_uris.clone()
  // }

  #[cfg(feature = "ssr")]
  pub async fn mongodb_setup(&mut self) -> mongodb::Client {

    if self.mongodb_uris.is_none() {
      warn!("there are NO mongodb addresses configured!");
      log!("falling back to localhost address: mongodb://127.0.0.1:27017");
      self.mongodb_uris = Some(String::from("mongodb://127.0.0.1:27017"));
    }

    let options = Self::parse_mongodb_uris(self.mongodb_uris.clone().unwrap()).await.unwrap();

    mongodb::Client::with_options(
      options
      // .tls(self.mongodb_tls)
    ).unwrap()

  }

  #[cfg(feature = "ssr")]
  async fn parse_mongodb_uris(uris: String) -> mongodb::error::Result<mongodb::options::ClientOptions> {
    log!("unparsed uris: {uris}");
    mongodb::options::ClientOptions::parse(uris).await
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
      mongodb_uris:       None,
      mongodb_tls:        None,
      mongodb_client:     None,
      quotes:             quote_queue::QuoteDocument::default(),
      documents:          document_queue::Documents::default(),
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
  use leptos::logging::*;
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
    let quote = collection.deserialize_current().unwrap_or({
      error!("quote can't be deserialized!");
      ReturnedQuote::default()
    });
    
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
      quote: Some(quote),
      expire
    })

  }

}

#[cfg(feature = "ssr")]
mod document_queue {

  use std::time;
  use std::collections::BTreeMap;
  use mongodb;


  pub type DocumentId = String;

  /// Collection of queued documents in cache, which were fetched by request and retained for a duration.
  /// Each document is referenced by its `DocumentId`. The value of this ID must be permenent so as to never store
  /// more than one copy of a particular document.
  #[derive(Clone, Default)] // TODO: Consider wrapping in Arc<Mutex>
  pub struct Documents {
    queue: Box<BTreeMap<DocumentId, DocumentComplete>>
  }

  /// Document and its associated caching information.
  /// 
  /// A singlet reveals how pouplar itself is to the cache and therefore how important it is for caching purposes.
  /// Additionally, for very popular documents, they may have an extended lifetime and update more frequently.
  /// Less popular documents are discarded from the cache if their expiration is reached, and while costs bandwidth and
  /// response timing later-on, helps to save space for documents which are far more important to cache.
  #[derive(Clone)]
  pub struct DocumentComplete {

    pub content: String,
    pub expiration: Option<time::Instant>, // If provided, document expires at the Instant.

    pub conceived: time::Instant, // The point in time where this document was cached.
    pub hits: usize, // Count of total hits (requests) for this document since it was cached.

    pub update_conceived: time::Instant, // the point in time where this document was last updated from Database.
    pub updates: usize, // Count of total updates for this document since it was cached.

    object_id: String, // Internal reference to MongoDB oid field.

  }

  impl Default for DocumentComplete {
    fn default() -> Self {
      DocumentComplete {
        content: String::new(),
        expiration: None,
        conceived: time::Instant::now(),
        hits: 0,
        update_conceived: time::Instant::now(),
        updates: 0,
        object_id: String::new(),
      }
    }
  }

}

#[cfg(test)]
#[cfg(feature = "ssr")]
mod tests {

  use super::*;
  use super::PeaceConfig;
  use std::{ env, time::Duration };
  // #[cfg(feature = "ssr")]
  use mongodb::options::ServerAddress;


  #[test]
  fn env_priming() {

    // * Test all values set non-default.
    let webauthn_id = "test";
    let webauthn_origin = "test.example.com:8080";
    let webauthn_name = "Testing";
    let webauthn_timeout = "60";
    let mongodb_uris = "mongodb://localhost:2025";
    let mongodb_tls_ca_cert = "";

    // * Vars are (temporarily) set for detection for `prime_envs()`
    unsafe {
      env::set_var(PEACE_WEBAUTHN_ID, webauthn_id);
      env::set_var(PEACE_WEBAUTHN_ORIGIN, webauthn_origin);
      env::set_var(PEACE_WEBAUTHN_NAME, webauthn_name);
      env::set_var(PEACE_WEBAUTHN_TIMEOUT, webauthn_timeout);
      env::set_var(PEACE_MONGODB_URIS, mongodb_uris);
      env::set_var(RAW_CA_CERT, mongodb_tls_ca_cert);
    }

    // * PeaceConfig should pull the vars above and populate itself with those parsed values.
    let config = PeaceConfig::prime_envs();

    assert_eq!(config.webauthn_rp_id.as_str(), webauthn_id);
    assert_eq!(config.webauthn_rp_origin.as_str(), webauthn_origin);
    assert_eq!(config.webauthn_rp_name, Some(webauthn_name.to_string()));
    assert_eq!(config.webauthn_timeout, Duration::from_secs(webauthn_timeout.parse::<u64>().unwrap()));
    assert_eq!(config.mongodb_uris, Some(String::from(mongodb_uris)));
    assert!(config.mongodb_tls.is_some());

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

    assert!(config.mongodb_uris.is_none()); // No mongodb servers specified by default.
    assert!(config.mongodb_tls.is_none()); // No tls by default.

  }

}
