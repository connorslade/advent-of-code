use std::env;

use anyhow::{Context, Result};
use scraper::Html;
use url::Url;

pub struct Session {
    token: String,
}

impl Session {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn from_env() -> Result<Self> {
        let token = env::var("AOC_TOKEN")?;
        Ok(Self::new(token))
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn verify(&self, address: &Url) -> Result<Option<SessionVerification>> {
        let body = ureq::get(address.as_str())
            .set("Cookie", &format!("session={}", self.token))
            .call()?
            .into_string()?;

        let document = Html::parse_document(&body);
        let user = match document.select(selector!(".user")).next() {
            Some(user) => user,
            None => return Ok(None),
        };
        let name = user
            .text()
            .next()
            .context("No username found")?
            .trim()
            .to_owned();

        Ok(Some(SessionVerification { name }))
    }
}

pub struct SessionVerification {
    pub name: String,
}
