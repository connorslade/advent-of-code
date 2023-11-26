use anyhow::Result;
use url::Url;

use crate::{commands::verify::verify_inner, session::Session, TOKEN_VAR};

pub fn token(session: &Option<Session>, token: String, url: &Url) -> Result<()> {
    if token.len() != 128 {
        anyhow::bail!("Invalid token length of {}, should be 128", token.len());
    }

    println!("[*] Validating session token...");
    let new_session = Session::new(token.clone());
    verify_inner(&new_session, url)?;
    println!("[*] Session token is valid.");

    if session.is_some() && session.as_ref().unwrap().is_from_env() {
        println!("[*] Updating session token");
    } else {
        println!("[*] Setting session token");
    }

    globalenv::set_var(TOKEN_VAR, &token)?;
    Ok(())
}
