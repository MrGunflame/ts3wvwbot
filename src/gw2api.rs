use bytes::Buf as _;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::result;
use std::sync::{Arc, Mutex};

pub type Result<T> = result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Guild Wars 2 API Client Wrapper
pub struct Client {
    endpoint: String,
    token: Option<String>,
    client: Arc<hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>>,
}

impl Client {
    pub fn new() -> Client {
        Client {
            endpoint: "https://api.guildwars2.com".to_owned(),
            token: None,
            client: Arc::new(
                hyper::Client::builder().build::<_, hyper::Body>(hyper_tls::HttpsConnector::new()),
            ),
        }
    }

    pub fn set_token(&mut self, token: Option<String>) {
        self.token = token
    }
}

impl Client {
    async fn get<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = (self.endpoint.clone() + endpoint).parse()?;

        println!("Sending {:?}", url);

        let resp = self.client.get(url).await?;

        let body = hyper::body::aggregate(resp).await?;

        let data = serde_json::from_reader(body.reader())?;

        Ok(data)
    }

    pub async fn account(&self) -> Result<Account> {
        self.get(&("/v2/account?access_token=".to_owned() + &self.token.as_ref().unwrap()))
            .await
    }

    pub async fn worlds(&self) -> Result<Vec<World>> {
        self.get(&"/v2/worlds?ids=all").await
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AccountAccess {
    None,
    PlayForFree,
    GuildWars2,
    HeartOfThorns,
    PathOfFire,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    id: String,
    age: u64,
    name: String,
    world: u32,
    guilds: Vec<String>,
    guild_leader: Vec<String>,
    created: String,
    access: Vec<AccountAccess>,
    commander: bool,
    fractal_level: u32,
    daily_ap: u32,
    monthly_ap: u32,
    wvw_rank: u32,
    // last_modified: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct World {
    id: u32,
    name: String,
    population: String,
}

pub fn valid_key(s: &str) -> bool {
    s.len() == 72
}
