
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use webauthn_rs;


const PEACE_WEBAUTHN_ID:      &'static str = "PEACE_WEBAUTHN_ID";
const PEACE_WEBAUTHN_ORIGIN:  &'static str = "PEACE_WEBAUTHN_ORIGIN";
const PEACE_WEBAUTHN_NAME:    &'static str = "PEACE_WEBAUTHN_NAME";
const PEACE_WEBAUTHN_TIMEOUT: &'static str = "PEACE_WEBAUTHN_TIMEOUT";
const PEACE_MONGODB_URIS:     &'static str = "PEACE_MONGODB_URIS";

#[derive(Clone)]
pub struct PeaceConfig {
  webauthn_rp_id:     String,
  webauthn_rp_origin: String,
  webauthn_rp_name:   Option<String>,
  webauthn_timeout:   std::time::Duration,
  mongodb_uris:       Vec<mongodb::options::ServerAddress>
}

impl PeaceConfig {

  pub fn prime_envs() -> Self {
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
        _ => ()
      }
    }

    config

  }

  fn parse_mongodb_uris(uris: String) -> Vec<mongodb::options::ServerAddress> {
    // TODO: more complicated logic may be implemented to prevent downstream errors from occuring due to bad input.
    let mut returnable = Vec::new();
    for each_uri in uris.split(',') {
      if let Ok(each_uri) = mongodb::options::ServerAddress::parse(each_uri) {
        returnable.push(each_uri);
      } else { leptos::logging::warn!("env {PEACE_MONGODB_URIS} specifies invalid uri: {each_uri}"); }
    }
    returnable
  }

}

impl Default for PeaceConfig {
  fn default() -> Self {
    PeaceConfig {
      webauthn_rp_id:     "127.0.0.1".to_string(),
      webauthn_rp_origin: "127.0.0.1:3000".to_string(),
      webauthn_rp_name:   None,
      webauthn_timeout:   webauthn_rs::DEFAULT_AUTHENTICATOR_TIMEOUT,
      mongodb_uris:       Vec::new()
    }
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

  }

}
