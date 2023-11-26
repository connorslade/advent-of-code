use anyhow::Result;
use url::Url;

use crate::session::Session;

pub fn verify(session: &Session, address: &Url) -> Result<()> {
    println!("[*] Verifying session token...");
    let verification = match session.verify(address) {
        Ok(Some(verification)) => verification,
        Ok(None) => {
            eprintln!("[E] Session token is invalid. Sign in again and update with `aoc token`.");
            return Ok(());
        }
        Err(err) => {
            eprintln!("[E] Failed to verify session: {}", err);
            return Ok(());
        }
    };

    println!("[*] Hello, {}!", verification.name);
    println!("[*] Session token is valid.");
    Ok(())
}
