use std::pin::Pin;
use std::future::Future;

pub type DbFuture<T> = Pin<Box<dyn Future<Output = Result<T, String>> + Send>>;