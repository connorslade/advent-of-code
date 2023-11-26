use anyhow::Result;

use crate::{
    args::{Args, TokenArgs},
    commands::verify::verify_inner,
    session::Session,
    TOKEN_VAR,
};

pub fn token(session: &Option<Session>, cmd: &TokenArgs, args: &Args) -> Result<()> {
    if cmd.token.len() != 128 {
        anyhow::bail!("Invalid token length of {}, should be 128", cmd.token.len());
    }

    println!("[*] Validating session token...");
    let new_session = Session::new(&cmd.token);
    verify_inner(&new_session, &args.address)?;
    println!("[*] Session token is valid.");

    if session.is_some() && session.as_ref().unwrap().is_from_env() {
        println!("[*] Updating session token");
    } else {
        println!("[*] Setting session token");
    }

    globalenv::set_var(TOKEN_VAR, &cmd.token)?;
    Ok(())
}
