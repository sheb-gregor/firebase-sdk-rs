use crate::errors;

pub struct AuthClient {}

impl AuthClient {
    pub fn new() -> errors::Result<Self> {
        Ok(AuthClient {})
    }
}
