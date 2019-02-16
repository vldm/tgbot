use dotenv::dotenv;
use env_logger;
use futures::future::lazy;
use futures::{Future, Stream};
use log;
use std::env;
use tgbot::methods::{GetMe, SendMessage};
use tgbot::types::{MessageData, MessageKind, UpdateKind};
use tgbot::Api;

fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGBOT_TOKEN").expect("TGBOT_TOKEN is not set");
    let proxy = env::var("TGBOT_PROXY").ok();
    let api = match proxy {
        Some(proxy) => Api::with_proxy(token, &proxy),
        None => Api::create(token),
    }
    .expect("Failed to create API");

    let mut rt =
        tokio::runtime::current_thread::Runtime::new().expect("Failed to create tokio runtime");

    rt.block_on(lazy(|| {
        tokio::executor::current_thread::spawn(api.execute(&GetMe).then(|x| {
            log::info!("getMe result: {:?}\n", x);
            Ok(())
        }));

        api.get_updates().for_each(move |update| {
            if let UpdateKind::Message(msg) = update.kind {
                log::info!("GOT A MESSAGE: {:?}\n", msg);
                if let MessageKind::Private { ref chat, .. } = msg.kind {
                    if let MessageData::Text(text) = msg.data {
                        let method = SendMessage::new(chat.id, text.data);
                        tokio::executor::current_thread::spawn(api.execute(&method).then(|x| {
                            log::info!("sendMessage result: {:?}\n", x);
                            Ok(())
                        }));
                    }
                }
            }
            Ok(())
        })
    }))
    .unwrap();
}