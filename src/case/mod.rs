pub mod ping;

use crate::resp;
use async_trait::async_trait;

#[async_trait]
pub trait Case: Clone + Send + 'static {
    fn name(&self) -> String;
    fn connections(&self) -> u64;
    fn requests(&self) -> u64;
    async fn run(&self, c: resp::Client);
}
