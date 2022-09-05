use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

pub type Jwt = TokenData<Claims>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub uname: String,
    pub exp: i64,
}

const SECRET_KEY: &str = env!("JWT_SECRET");

impl Claims {
    pub fn decode(token: &str) -> anyhow::Result<Jwt> {
        let jwt = jsonwebtoken::decode(
            token,
            &DecodingKey::from_secret(SECRET_KEY.as_bytes()),
            &Validation::default(),
        )?;

        Ok(jwt)
    }

    pub fn encode(&self) -> anyhow::Result<String> {
        Ok(jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(SECRET_KEY.as_bytes()),
        )?)
    }
}

impl From<String> for Claims {
    fn from(uname: String) -> Self {
        Self {
            uname,
            exp: (Utc::now() + Duration::days(365 / 2)).timestamp(),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::ensure;

    use crate::jwt::Claims;

    #[test]
    fn default_validation() -> anyhow::Result<()> {
        let claims = Claims::decode(&Claims::from(String::from("uname")).encode()?)?.claims;
        ensure!(claims.uname == "uname");
        ensure!(claims.exp > 0);
        Ok(())
    }
}
