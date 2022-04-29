use futures::Future;
use std::pin::Pin;

pub type AsyncResult<O, E> = Pin<Box<dyn Future<Output = Result<O, E>> + Send>>;
