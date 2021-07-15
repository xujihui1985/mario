use async_trait::async_trait;

pub mod errors;

#[async_trait]
pub trait Collector {
    async fn collect(&self);
    fn get_name(&self) -> String;
}
