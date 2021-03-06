use crate::methods::Request;
use failure::Error;
use futures::Future;

mod hyper;

pub(crate) use self::hyper::{default_executor, proxy_executor};

pub(crate) trait Executor: Send + Sync {
    fn execute(&self, req: Request) -> Box<Future<Item = Vec<u8>, Error = Error> + Send>;
}
