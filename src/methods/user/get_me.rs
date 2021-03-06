use crate::{methods::method::*, types::User};
use failure::Error;

/// Returns basic information about the bot in form of a User object
#[derive(Clone, Copy, Debug)]
pub struct GetMe;

impl Method for GetMe {
    type Response = User;

    fn get_request(&self) -> Result<RequestBuilder, Error> {
        RequestBuilder::empty("getMe")
    }
}
