use std::{collections::HashMap, fmt::Debug};

/**
 * Different credential types supported by opensearch-client.
 */
#[derive(Clone)]
pub enum Credentials {
    /// HTTP basic auth credentials
    Basic {
        username: String,
        password: Option<String>,
    },
    /// HTTP basic auth credentials, pre-encoded
    EncodedBasic(String),
    /// HTTP Bearer token auth
    Token(String),
    /// HTTP Api Key
    ApiKey { prefix: Option<String>, key: String },
}

impl Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Basic { username, .. } => {
                f.write_fmt(format_args!("Basic(username={},password=***)", username))
            }
            Self::EncodedBasic(_) => f.write_str("EncodedBasic(***)"),
            Self::Token(_) => f.write_str("Token(***)"),
            Self::ApiKey { prefix, key: _ } => f.write_fmt(format_args!(
                "ApiKey(prefix={},key=***)",
                prefix.as_deref().unwrap_or(""),
            )),
        }
    }
}

impl TryFrom<HashMap<String, String>> for Credentials {
    type Error = crate::Error;

    fn try_from(value: HashMap<String, String>) -> Result<Self, Self::Error> {
        if let Some(token) = value.get("token") {
            Ok(Self::Token(token.to_owned()))
        } else if let (Some(username), password) = (value.get("username"), value.get("password")) {
            Ok(Self::Basic {
                username: username.to_owned(),
                password: password.map(|s| s.to_owned()),
            })
        } else if let Some(auth) = value.get("auth") {
            Ok(Self::EncodedBasic(auth.to_owned()))
        } else if let Some(key) = value.get("api-key") {
            Ok(Self::ApiKey {
                prefix: None,
                key: key.to_owned(),
            })
        } else {
            Err(crate::Error::CredentialsConfigError(
                "Credentials either have to contain a token or username/password".to_string(),
            ))
        }
    }
}
