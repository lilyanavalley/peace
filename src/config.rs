
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
  pub async fn prime_qotd(&mut self, mongodb: &mongodb::Client) -> mongodb::error::Result<()> {
    self.quotes = quote_server::fetch(mongodb).await?;
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

  #[cfg(feature = "ssr")]
  pub fn get_document(&self, document_id: &str) -> Option<document_queue::DocumentComplete> {
    self.documents.get(&document_id.to_string())
  }

  #[cfg(feature = "ssr")]
  pub fn cleanup_documents(&mut self) -> usize {
    self.documents.cleanup_expired()
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
  use leptos::logging::*;
  use serde::{Deserialize, Serialize};

  pub type DocumentId = String;

  /// Collection of queued documents in cache, which were fetched by request and retained for a duration.
  /// Each document is referenced by its `DocumentId`. The value of this ID must be permenent so as to never store
  /// more than one copy of a particular document.
  #[derive(Clone, Default)]
  pub struct Documents {
    queue: Box<BTreeMap<DocumentId, DocumentComplete>>
  }

  impl Documents {

    pub fn get(&self, id: &DocumentId) -> Option<DocumentComplete> {
      if let Some(mut doc) = self.queue.get(id).cloned() {
        // Increment hit counter
        doc.hits += 1;
        Some(doc)
      } else {
        None
      }
    }

    pub fn insert(&mut self, id: DocumentId, document: DocumentComplete) {
      self.queue.insert(id, document);
    }

    pub fn update(&mut self, id: &DocumentId, content: String, object_id: String) -> bool {
      if let Some(doc) = self.queue.get_mut(id) {
        doc.content = content;
        doc.object_id = object_id;
        doc.update_conceived = time::Instant::now();
        doc.updates += 1;
        // Extend expiration for popular documents
        if doc.hits > 10 {
          doc.expiration = Some(time::Instant::now() + time::Duration::from_secs(86400 * 7)); // 7 days for popular docs
        }
        true
      } else {
        false
      }
    }

    pub fn is_expired(&self, id: &DocumentId) -> bool {
      if let Some(doc) = self.queue.get(id) {
        if let Some(expiration) = doc.expiration {
          time::Instant::now() > expiration
        } else {
          false
        }
      } else {
        true // Document not found is considered "expired"
      }
    }

    pub fn cleanup_expired(&mut self) -> usize {
      let before_count = self.queue.len();
      self.queue.retain(|_id, doc| {
        if let Some(expiration) = doc.expiration {
          time::Instant::now() <= expiration
        } else {
          true // Keep documents without expiration
        }
      });
      let removed = before_count - self.queue.len();
      if removed > 0 {
        log!("Cleaned up {removed} expired documents from cache");
      }
      removed
    }

    pub fn size(&self) -> usize {
      self.queue.len()
    }

    pub fn get_popular_documents(&self, threshold: usize) -> Vec<(DocumentId, usize)> {
      self.queue
        .iter()
        .filter(|(_, doc)| doc.hits >= threshold)
        .map(|(id, doc)| (id.clone(), doc.hits))
        .collect()
    }

  }

  /// Document stored in MongoDB
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct DocumentRecord {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub document_id: String,
    pub content: String,
    pub title: Option<String>,
    pub created_at: mongodb::bson::DateTime,
    pub updated_at: mongodb::bson::DateTime,
    pub version: i32,
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
    pub title: Option<String>,
    pub expiration: Option<time::Instant>, // If provided, document expires at the Instant.

    pub conceived: time::Instant, // The point in time where this document was cached.
    pub hits: usize, // Count of total hits (requests) for this document since it was cached.

    pub update_conceived: time::Instant, // the point in time where this document was last updated from Database.
    pub updates: usize, // Count of total updates for this document since it was cached.

    object_id: String, // Internal reference to MongoDB oid field.
    pub version: i32, // Version number from database

  }

  impl Default for DocumentComplete {
    fn default() -> Self {
      DocumentComplete {
        content: String::new(),
        title: None,
        expiration: Some(time::Instant::now() + time::Duration::from_secs(86400)), // Default 24 hour expiration
        conceived: time::Instant::now(),
        hits: 0,
        update_conceived: time::Instant::now(),
        updates: 0,
        object_id: String::new(),
        version: 0,
      }
    }
  }

  impl From<DocumentRecord> for DocumentComplete {
    fn from(record: DocumentRecord) -> Self {
      DocumentComplete {
        content: record.content,
        title: record.title,
        expiration: Some(time::Instant::now() + time::Duration::from_secs(86400)), // 24 hours
        conceived: time::Instant::now(),
        hits: 0,
        update_conceived: time::Instant::now(),
        updates: 0,
        object_id: record.id.map(|oid| oid.to_hex()).unwrap_or_default(),
        version: record.version,
      }
    }
  }

}

/// Document server module for managing cached markdown documents
/// 
/// # Usage Example:
/// ```rust
/// // In your Leptos server function:
/// #[server(GetDocument, "/api")]
/// pub async fn get_document(document_id: String) -> Result<Option<String>, ServerFnError> {
///     use actix_web::web::Data;
///     use std::sync::{Arc, Mutex};
/// 
///     let config = expect_context::<Data<Arc<Mutex<PeaceConfig>>>>();
///     let mongodb = expect_context::<Data<mongodb::Client>>();
/// 
///     match document_server::get_document(config.clone(), &mongodb, &document_id).await {
///         Ok(Some(doc)) => {
///             // Render markdown to HTML using comrak
///             let html = comrak::markdown_to_html(&doc.content, &comrak::Options::default());
///             Ok(Some(html))
///         },
///         Ok(None) => Ok(None),
///         Err(e) => Err(ServerFnError::ServerError(format!("Database error: {}", e)))
///     }
/// }
/// 
/// // Periodic maintenance (call from a scheduled task):
/// document_server::run_maintenance_task(config.clone(), &mongodb).await;
/// ```
pub mod document_server {

  use super::document_queue::*;
  use mongodb;
  use leptos::logging::*;
  use std::sync::{Arc, Mutex};

  const DOCUMENTS_DB_DATABASE: &'static str = "profile";
  const DOCUMENTS_DB_COLLECTION: &'static str = "documents";

  /// Fetch document from cache or database
  pub async fn get_document(
    config: Arc<Mutex<super::PeaceConfig>>,
    mongodb: &mongodb::Client,
    document_id: &str,
  ) -> mongodb::error::Result<Option<DocumentComplete>> {
    
    // First check cache
    {
      let mut config_guard = config.lock().unwrap();
      
      // Clean up expired documents periodically
      config_guard.documents.cleanup_expired();
      
      if let Some(doc) = config_guard.documents.get(&document_id.to_string()) {
        if !config_guard.documents.is_expired(&document_id.to_string()) {
          log!("Document '{}' served from cache (hits: {})", document_id, doc.hits);
          return Ok(Some(doc));
        } else {
          log!("Document '{}' expired in cache, fetching from database", document_id);
        }
      }
    }

    // Fetch from database
    log!("Fetching document '{}' from database", document_id);
    match fetch_from_database(mongodb, document_id).await? {
      Some(record) => {
        let document = DocumentComplete::from(record);
        
        // Store in cache
        {
          let mut config_guard = config.lock().unwrap();
          config_guard.documents.insert(document_id.to_string(), document.clone());
        }
        
        log!("Document '{}' cached successfully", document_id);
        Ok(Some(document))
      },
      None => {
        log!("Document '{}' not found in database", document_id);
        Ok(None)
      }
    }
  }

  /// Check if document needs updating by comparing versions
  pub async fn check_and_update_document(
    config: Arc<Mutex<super::PeaceConfig>>,
    mongodb: &mongodb::Client,
    document_id: &str,
  ) -> mongodb::error::Result<bool> {
    
    let current_version = {
      let config_guard = config.lock().unwrap();
      config_guard.documents.get(&document_id.to_string())
        .map(|doc| doc.version)
    };

    if let Some(cached_version) = current_version {
      // Check database for newer version
      if let Some(db_version) = get_document_version(mongodb, document_id).await? {
        if db_version > cached_version {
          log!("Document '{}' has newer version ({} -> {}), updating cache", 
               document_id, cached_version, db_version);
          
          // Fetch and update
          if let Some(record) = fetch_from_database(mongodb, document_id).await? {
            let mut config_guard = config.lock().unwrap();
            config_guard.documents.update(
              &document_id.to_string(), 
              record.content.clone(), 
              record.id.map(|oid| oid.to_hex()).unwrap_or_default()
            );
            return Ok(true);
          }
        }
      }
    }
    
    Ok(false)
  }

  /// Fetch document from MongoDB
  async fn fetch_from_database(
    mongodb: &mongodb::Client,
    document_id: &str,
  ) -> mongodb::error::Result<Option<DocumentRecord>> {
    
    let collection = mongodb
      .database(DOCUMENTS_DB_DATABASE)
      .collection::<DocumentRecord>(DOCUMENTS_DB_COLLECTION);

    let filter = mongodb::bson::doc! { "document_id": document_id };
    
    collection.find_one(filter).await
  }

  /// Get document version from database without fetching full content
  async fn get_document_version(
    mongodb: &mongodb::Client,
    document_id: &str,
  ) -> mongodb::error::Result<Option<i32>> {
    
    let collection = mongodb
      .database(DOCUMENTS_DB_DATABASE)
      .collection::<mongodb::bson::Document>(DOCUMENTS_DB_COLLECTION);

    let filter = mongodb::bson::doc! { "document_id": document_id };
    let projection = mongodb::bson::doc! { "version": 1, "_id": 0 };
    
    if let Some(doc) = collection.find_one(filter).projection(projection).await? {
      Ok(doc.get_i32("version").ok())
    } else {
      Ok(None)
    }
  }

  /// Preload popular documents into cache
  pub async fn preload_popular_documents(
    config: Arc<Mutex<super::PeaceConfig>>,
    mongodb: &mongodb::Client,
  ) -> mongodb::error::Result<usize> {
    
    log!("Preloading popular documents...");
    
    let collection = mongodb
      .database(DOCUMENTS_DB_DATABASE)
      .collection::<DocumentRecord>(DOCUMENTS_DB_COLLECTION);

    // Fetch documents that might be popular (you can adjust this query based on your needs)
    let mut cursor = collection.find(mongodb::bson::doc! {}).limit(10).await?;
    
    let mut documents = Vec::new();
    while cursor.advance().await? {
      if let Ok(doc) = cursor.deserialize_current() {
        documents.push(doc);
      }
    }
    let count = documents.len();
    
    {
      let mut config_guard = config.lock().unwrap();
      for record in documents {
        let document = DocumentComplete::from(record.clone());
        config_guard.documents.insert(record.document_id, document);
      }
    }
    
    log!("Preloaded {} documents into cache", count);
    Ok(count)
  }

  /// Background task to refresh popular documents
  pub async fn refresh_popular_documents(
    config: Arc<Mutex<super::PeaceConfig>>,
    mongodb: &mongodb::Client,
  ) -> mongodb::error::Result<usize> {
    
    let popular_docs = {
      let config_guard = config.lock().unwrap();
      config_guard.documents.get_popular_documents(5) // Documents with 5+ hits
    };

    let mut refreshed = 0;
    for (doc_id, hits) in popular_docs {
      log!("Refreshing popular document '{}' ({} hits)", doc_id, hits);
      if check_and_update_document(config.clone(), mongodb, &doc_id).await? {
        refreshed += 1;
      }
    }

    if refreshed > 0 {
      log!("Refreshed {} popular documents", refreshed);
    }

    Ok(refreshed)
  }

  /// Spawn a background task to periodically refresh documents and clean up expired ones
  /// This should be called from your actix-web server setup
  pub async fn run_maintenance_task(
    config: Arc<Mutex<super::PeaceConfig>>,
    mongodb: &mongodb::Client,
  ) -> mongodb::error::Result<()> {
    
    log!("Running document cache maintenance...");
    
    // Clean up expired documents
    {
      let mut config_guard = config.lock().unwrap();
      let cleaned = config_guard.documents.cleanup_expired();
      if cleaned > 0 {
        log!("Maintenance: cleaned {} expired documents", cleaned);
      }
    }
    
    // Refresh popular documents
    let refreshed = refresh_popular_documents(config.clone(), mongodb).await?;
    if refreshed > 0 {
      log!("Maintenance: refreshed {} popular documents", refreshed);
    }
    
    // Log cache statistics
    {
      let config_guard = config.lock().unwrap();
      let cache_size = config_guard.documents.size();
      let popular = config_guard.documents.get_popular_documents(5);
      log!("Cache stats: {} documents cached, {} popular", cache_size, popular.len());
    }

    Ok(())
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
