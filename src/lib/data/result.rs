use std::{pin::Pin};
use futures::Future;

pub type AsyncResult<O, E> = Pin<Box<dyn Future<Output = Result<O, E>>>>;
