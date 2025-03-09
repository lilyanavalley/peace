
use serde::{Deserialize, Serialize};


/// Key for the environment variable pointing to a mongodb server for testing.
const ENV_MONGODB_URI:      &'static str = "MONGODB_URI";

/// Generalized website settings.
const GENERAL_DATABASE:     &'static str = "general";
/// Settings for the *Ask Me Anything* page type.
const AMAS_DATABASE:        &'static str = "ama";
/// Settings for the *Contact Me* page type.
const CONTACT_DATABASE:     &'static str = "contact";
/// Settings for the *Portfolio* page type.
const PORTFOLIO_DATABASE:   &'static str = "portfolio";

/// Data related to the site owner.
const OWNER_COLLECTION:     &'static str = "generalOwner";
/// Site theme settings.
const THEME_COLLECTION:     &'static str = "generalTheme";
/// Routes and their pages.
const ROUTES_COLLECTION:    &'static str = "routes";
/// Submissions to *Ask Me Anything*.
const AMAS_COLLECTION:      &'static str = "amaSubmissions";
/// Portfolio showcasing collection.
const PORTFOLIO_COLLECTION: &'static str = "portfolioEntries";


#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GeneralOwner {

    /// Name given by the owner of the site to represent their profile.
    name:           String,

    /// Point in time of the birth of the owner. `None` to disable.
    age:            Option<std::time::SystemTime>,

    /// URI pointing to the Owner's profile picture, which could be remote or local to the site.
    avatar:         String,

    /// Profile description.
    description:    String,

    /// (Short) tagline of the Owner's profile.
    tagline:        String,

    /// Profile links to other websites, social media, etc.
    links:          Option<Vec<Links>>

}

impl Default for GeneralOwner {
    fn default() -> Self {
        GeneralOwner { 
            name: "Peace Web".to_string(), 
            age: None, 
            avatar: "assets/avatar.svg".to_string(),
            description: String::new(), 
            tagline: String::new(),
            links: None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Links {

    /// Link URI.
    uri:    String,

    /// Phosphor Icon identifier, e.g.: `ph ph-acorn`
    icon:   Option<String>,

    /// Friendly text for this link.
    text:   String

}
