use chrono::{DateTime, Utc};
use jwt_simple::prelude::*;
use std::collections::HashSet;
use tonic::service::Interceptor;
use tonic::{Request, Status};

#[allow(unused)]
const JWT_ISS: &str = "chat-server";
#[allow(unused)]
const JWT_AUD: &str = "chat_web";

#[derive(Debug, Clone)]
pub struct DecodingKey(Ed25519PublicKey);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: i64,
    pub ws_id: i64,
    pub fullname: String,
    pub email: String,
    #[serde(skip)]
    pub password_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[allow(unused)]
impl DecodingKey {
    pub fn load(pem: &str) -> Result<Self, jwt_simple::Error> {
        Ok(Self(Ed25519PublicKey::from_pem(pem)?))
    }

    pub fn verify(&self, token: &str) -> Result<User, jwt_simple::Error> {
        let opts = VerificationOptions {
            allowed_issuers: Some(HashSet::from_strings(&[JWT_ISS])),
            allowed_audiences: Some(HashSet::from_strings(&[JWT_AUD])),
            ..Default::default()
        };
        let claims = self.0.verify_token(token, Some(opts))?;
        Ok(claims.custom)
    }
}

impl Interceptor for DecodingKey {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let token = request
            .metadata()
            .get("authorization")
            .and_then(|h| h.to_str().ok());
        let user = match token {
            Some(bearer) => {
                let token = bearer
                    .strip_prefix("Bearer ")
                    .ok_or_else(|| Status::unauthenticated("invalid token format"))?;
                self.verify(token)
                    .map_err(|e| Status::unauthenticated(e.to_string()))?
            }
            None => return Err(Status::unauthenticated("invalid token")),
        };
        request.extensions_mut().insert(user);
        Ok(request)
    }
}
