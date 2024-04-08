use crate::config;
use crate::infrastructure::error::InfrastructureError;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::ops::Add;
use std::time::Duration;
use std::{
    collections::BTreeMap,
    time::{SystemTime, UNIX_EPOCH},
};
use tonic::{Request, Status};
#[derive(Default)]
pub struct AuthDomain {}

impl AuthDomain {
    // Auth関連の処理
    pub fn get_token(
        &self,
        request_metadata: &tonic::metadata::MetadataMap,
    ) -> Result<String, Status> {
        let token = request_metadata
            .get("authorization")
            .ok_or(Status::unauthenticated("No access token specified"))?
            .to_str()
            .map_err(|_| Status::unauthenticated("Invalid access token"))?;
        Ok(token.to_string())
    }

    fn generate_claims(
        &self,
        user_id: i64,
    ) -> Result<BTreeMap<&'static str, String>, InfrastructureError> {
        let mut claims: BTreeMap<&str, String> = BTreeMap::new();

        claims.insert("sub", user_id.to_string());

        let current_timestamp = SystemTime::now().duration_since(UNIX_EPOCH);
        let exp = SystemTime::now()
            .add(Duration::from_secs(3600))
            .duration_since(UNIX_EPOCH);

        claims.insert("iat", current_timestamp.unwrap().as_secs().to_string());
        claims.insert("exp", exp.unwrap().as_secs().to_string());
        Ok(claims)
    }

    pub fn generate_token(&self, user_id: i64) -> Result<String, InfrastructureError> {
        let app_key: String = config::CONFIG.app_key.clone();
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(app_key.as_bytes()).expect("failed to create key from app key");
        let claims = self.generate_claims(user_id)?;
        let acces_token = claims.sign_with_key(&key)?;
        Ok(acces_token)
    }

    pub fn verify_token(&self, token: &str) -> Result<BTreeMap<String, String>, Status> {
        let app_key: String = config::CONFIG.app_key.clone();

        let key: Hmac<Sha256> = Hmac::new_from_slice(app_key.as_bytes())
            .map_err(|_| Status::failed_precondition("failed to create key"))?;
        token
            .verify_with_key(&key)
            .map_err(|_| Status::failed_precondition("failed to verify"))
    }

    pub fn auth<T>(&self, request: Request<T>) -> Result<i64, Status> {
        let token = self.get_token(request.metadata())?;
        let claim = self.verify_token(&token)?;
        let user_id = claim["sub"].parse::<i64>().map_err(|_| {
            Status::unauthenticated("Invalid access token: user_id is not found in claim")
        })?;
        Ok(user_id)
    }
}
