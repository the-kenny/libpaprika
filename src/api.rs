use futures::StreamExt;
use futures::TryStreamExt;
use log::debug;
use log::info;
use reqwest::Client;
use serde::Deserialize;

use crate::Recipe;

const API_ROOT: &str = "https://www.paprikaapp.com/api/v2";

pub type Token = String;

#[derive(Deserialize, Debug)]
struct ApiResult<T> {
    result: T,
}

pub async fn login(client: &Client, email: &str, password: &str) -> Result<Token, reqwest::Error> {
    #[derive(Deserialize)]
    struct TokenResponse {
        pub token: String,
    }

    let response: ApiResult<TokenResponse> = client
        .post(format!("{API_ROOT}/account/login"))
        .form(&[("email", email), ("password", password)])
        .send()
        .await?
        .json()
        .await?;

    Ok(response.result.token)
}

pub async fn recipes(client: &Client, token: &Token) -> Result<Vec<Recipe>, reqwest::Error> {
    #[derive(Deserialize, Debug)]
    struct RecipeEntry {
        uid: String,
        // hash: String,
    }

    type RecipesResponse = Vec<RecipeEntry>;

    let response: ApiResult<RecipesResponse> = client
        .get(format!("{API_ROOT}/sync/recipes"))
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;

    let recipes = futures::stream::iter(response.result.into_iter())
        .map(move |re| recipe(client, token, re.uid))
        .buffer_unordered(8)
        .try_collect::<Vec<Recipe>>()
        .await?;

    Ok(recipes)
}

pub async fn recipe(
    client: &Client,
    token: &Token,
    uid: impl AsRef<str>,
) -> Result<Recipe, reqwest::Error> {
    let uid = uid.as_ref();
    info!("Fetching Recipe {uid}");

    let response: ApiResult<Recipe> = client
        .get(format!("{API_ROOT}/sync/recipe/{uid}"))
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;

    debug!("Result for Recipe {uid}: {response:?}");

    Ok(response.result)
}

#[cfg(test)]
mod test {
    use std::env;

    use reqwest::Client;

    use super::{login, recipes, Token};

    async fn env_token() -> Option<Token> {
        env::var("PAPRIKA_TOKEN").ok()
    }

    #[tokio::test]
    async fn test_login() {
        let (Ok(email), Ok(password)) = (env::var("PAPRIKA_EMAIL"), env::var("PAPRIKA_PASSWORD")) else {
        return
      };

        login(&Client::new(), &email, &password).await.unwrap();
    }

    #[tokio::test]
    async fn test_recipes() {
        let Some(token) = env_token().await else {
          return
        };

        let rs = recipes(&Client::new(), &token).await.unwrap();
        assert_ne!(rs.len(), 0);
    }
}
