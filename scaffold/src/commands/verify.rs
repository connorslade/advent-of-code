use anyhow::{bail, Result};
use colored::Colorize;
use url::Url;

use crate::session::{Session, SessionVerification};

pub fn verify(session: &Session, address: &Url) -> Result<()> {
    println!("[*] Verifying session token...");
    let verification = verify_inner(session, address)?;

    println!("[*] Hello, {}!", verification.name.underline());
    println!("{}", "[*] Session token is valid.".green());
    Ok(())
}

pub fn verify_inner(session: &Session, address: &Url) -> Result<SessionVerification> {
    match session.verify(address) {
        Ok(Some(verification)) => Ok(verification),
        Ok(None) => {
            bail!("[E] Session token is invalid. Sign in again and update with `aoc token`.")
        }
        Err(err) => bail!("[E] Failed to verify session: {}", err),
    }
}
