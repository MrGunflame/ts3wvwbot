mod config;
mod database;
mod gw2api;

use async_trait::async_trait;
use database::DBHandler;
use tokio::signal;
use ts3::client::ServerNotifyRegister;
use ts3::event::EventHandler;

extern crate async_trait;
extern crate bytes;
extern crate tokio;
extern crate ts3;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = ts3::Client::new("localhost:10011").await?;
    client.set_event_handler(Handler::new().await.unwrap());

    // Login and connect to server with given port
    client.login("serveradmin", "yewnC+Xr").await.unwrap();
    client.use_port(9987).await?;

    // Register for listening on server and textprivate events
    for event in vec![
        ServerNotifyRegister::Server,
        ServerNotifyRegister::TextPrivate,
    ] {
        match client.servernotifyregister(event).await {
            Ok(_) => (),
            Err(err) => {
                eprintln!("[ERROR] Failed to register event notify: {}", err);
                return Ok(());
            }
        }
    }

    // Wait for SIGTERM
    signal::ctrl_c().await.unwrap();
    client.quit().await.unwrap();

    Ok(())
}

#[derive(Clone)]
struct Handler {
    db: DBHandler,
}

impl Handler {
    async fn new() -> Result<Handler, sqlx::Error> {
        Ok(Handler {
            db: DBHandler::new().await?,
        })
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn cliententerview(&self, c: ts3::Client, ev: ts3::RawResp) {
        // let clid = ev.items[0].get("clid").unwrap().as_ref().unwrap().parse();
        let clid = get_field!(&ev, "clid", usize);
        sendtextmessage!(c, clid, "Hello World!");
    }

    async fn textmessage(&self, c: ts3::Client, ev: ts3::event::TextMessage) {
        match ev.msg.as_str() {
            // Move to guild section
            msg if msg.starts_with("!guild ") => match msg.replacen("!guild ", "", 1).as_str() {
                "add" => {}
                "rm" => {}
                "list" => {}
                _ => (),
            },
            // Give guild commands
            "!guild" => {
                sendtextmessage!(
                    c,
                    ev.invokerid,
                    "Guild Commands:\nadd <Guild Name> [Guild Channel ID] : Add a new guild\nrm <Guild Name> : Remove a guild"
                );
            }

            // Expect to be api token
            _ => {}
        }
    }
}

/// Use the sendtextmessage command to send a direct text message to the client with id `clid`.
#[macro_export]
macro_rules! sendtextmessage {
    ($c:expr, $clid:expr, $msg:expr) => {{
        $c.send(format!(
            "sendtextmessage targetmode=1 target={} msg={}",
            $clid,
            &$msg.replace(" ", "\\s").replace("\n", "\\n")
        ))
        .await
        .unwrap();
    }};
}

/// Extract the value from a field, expecting that it is always found in the first hashmap and that it is not None.
#[macro_export]
macro_rules! get_field {
    ($raw:expr, $name:expr, $t:ty) => {{
        $raw.items[0]
            .get($name)
            .unwrap()
            .as_ref()
            .unwrap()
            .parse::<$t>()
            .unwrap()
    }};
}
