use std::future::Future;

use tokio::runtime::Runtime;

pub fn blocking_call<F, T>(f: F) -> T
where
    F: Future<Output = T>,
{
    Runtime::new().unwrap().block_on(f)
}
