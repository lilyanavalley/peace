
use std::net::IpAddr;
use serde::{ Serialize, Deserialize };
// use webauthn_rs::prelude::Passkey;
use chrono:: { DateTime, Offset };

// pub const PROFILE_DB_DATABASE:  &'static str = "profile";
// pub const PROFILE_DB_COLLECTION_QUOTES:  &'static str = "quotes";

const ACCESS_DB_DATABASE:  &'static str = "access";
const ACCESS_DB_COLLECTION_RATELIMIT: &'static str = "ratelimit";

const AUTH_DB_DATABASE:  &'static str = "authentication";
const AUTH_DB_COLLECTION_USERS:  &'static str = "users";
const AUTH_DB_COLLECTION_PASSKEYS: &'static str = "passkeys";
const AUTH_DB_COLLECTION_SESSIONS: &'static str = "sessions";


#[derive(Serialize, Deserialize)]
/// A quote to display in the favorite quotes component.
pub struct FavoriteQuotes {
  id:         u32,
  quotation:  String,
  citation:   Option<String>
}

/// One user account.
/// 
/// Note the use of u32 in `id`. If you don't understand upon first glance, u32 has a maximum of 4,294,967,295.
/// 
/// A tad over 4 billion sounds like a lot, but keep in mind that *means* you're limited to 4 billion users.
/// Ideally not an issue in real-world situations, but it might be worth something to use something like a UUID...
// TODO: consider change explained in doc. ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
#[derive(Serialize, Deserialize)]
pub struct User {
  id:         u32,
  username:   String,
  pronouns:   String,
  comment:    String,
}

/// A 'Session', a simple audit trail for user authentication history.
/// 
/// Note the use of u32 in `id`. If you don't understand upon first glance, u32 has a maximum of 4,294,967,295.
/// 
/// A tad over 4 billion sounds like a lot, but keep in mind that *means* you're limited to 4 billion users.
/// Ideally not an issue in real-world situations, but it might be worth something to use something like a UUID...
// TODO: consider change explained in doc. ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
#[derive(Serialize, Deserialize)]
pub struct UserSession {
  user_id:      u32,              // references `User.id`.
  ip:           IpAddr,           // origin IP.
  ip_whois:     String,           // url to origin whois report.
  ip_range:     (IpAddr, IpAddr), // origin IP block range, (start, end).
  geolocation:  String,           // human-friendly geolocation of IP.
  useragent:    String,           // human-friendly client type, OS, browser.
}

/// A passkey for a user.
/// 
/// Note the use of u32 in `id`. If you don't understand upon first glance, `u32` has a maximum value of 
/// 4,294,967,295.
/// 
/// A tad over 4 billion sounds like a lot, but keep in mind that *means* you're limited to 4 billion users.
/// Ideally not an issue in real-world situations, but it might be worth something like UUIDs...
/// 
// TODO: consider change explained in doc. ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
#[derive(Serialize, Deserialize)]
pub struct UserPasskey {
  id:                 u32,
  name:               String,
  // data:               Passkey,  
  status:             PasskeyValidity,
  count_blocked:      u32
}

impl UserPasskey {
  
  pub fn internal_id(&self) -> &u32 {
    &self.id
  }
  
  // pub fn status(&self) -> &PasskeyValidity {
  //   &self.status
  // }
  
  pub fn name(&self) -> &str {
    &self.name
  }
  
  // pub fn credential_id(&self) -> &webauthn_rs::prelude::CredentialID {
  //   &self.data.cred_id()
  // }
  
  // pub fn credential_algorithm(&self) -> &webauthn_rs::prelude::COSEAlgorithm {
  //   &self.data.cred_algorithm()
  // }

  /// # of blocked key usage attempts (after revocation.)
  pub fn count_blocked(&self) -> &u32 {
    &self.count_blocked
  }

}

/// Whether key is valid.
/// 
// TODO: Document further.
#[derive(Serialize, Deserialize)]
pub enum PasskeyValidity {
  Valid,
  Invalid(PasskeyRevocation)
}

/// Revocation of a Passkey.
/// 
/// Includes complementary information, such as comments or date of action.
#[derive(Serialize, Deserialize)]
enum PasskeyRevocation {
  /// Key met its expiration.
  Expired,
  /// Key replaced; superseded by another.
  Replaced { superseding_keyid: Option<u32> },
  /// Key known to be compromised.
  Compromised { date_assessed: u64, comment: Option<String> },
  /// User-specified revocation.
  Other { comment: Option<String>, icon: char }
}

// #[derive(Serialize, Deserialize)]
// pub struct Ratelimit {
//   realip:   Option<IpAddr>,
//   peerip:   Option<IpAddr>,
//   response: super::limit::RateLimitResponse,
// }
