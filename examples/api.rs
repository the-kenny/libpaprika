use std::{env, error::Error};

use libpaprika::api::*;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let (Ok(email), Ok(password)) = (env::var("PAPRIKA_EMAIL"), env::var("PAPRIKA_PASSWORD")) else {
    eprintln!("Specify PAPRIKA_USERNAME and PAPRIKA_PASSWORD as environment variables");
    return Ok(());
  };

    let client = reqwest::Client::new();

    let token = login(&client, &email, &password).await?;
    let recipes = recipes(&client, &token).await?;

    for recipe in recipes {
        println!("- {} ({})", recipe.name, recipe.categories.join(", "))
    }

    Ok(())
}
