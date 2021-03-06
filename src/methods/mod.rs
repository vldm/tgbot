mod answer;
mod chat;
mod chat_member;
mod game;
mod get_file;
mod message;
mod method;
mod passport;
mod send;
mod sticker;
mod updates;
mod user;

pub use self::{
    answer::*, chat::*, chat_member::*, game::*, get_file::*, message::*, method::*, passport::*, send::*, sticker::*,
    updates::*, user::*,
};
