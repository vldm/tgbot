use dotenv::dotenv;
use env_logger;
use log;
use std::env;
use tgbot::{run_server, types::Update, Api, UpdateHandler};

struct Handler;

impl UpdateHandler for Handler {
    fn handle(&mut self, _: &Api, update: Update) {
        log::info!("got an update: {:?}\n", update);
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGBOT_TOKEN").expect("TGBOT_TOKEN is not set");
    let proxy = env::var("TGBOT_PROXY").ok();
    let api = match proxy {
        Some(proxy) => Api::with_proxy(token, &proxy),
        None => Api::new(token),
    }
    .expect("Failed to create API");

    run_server(api, ([127, 0, 0, 1], 8080), "/", Handler);
}
