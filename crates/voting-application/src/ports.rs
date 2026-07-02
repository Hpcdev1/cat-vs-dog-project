
//  Les contrats de l'architecture hexagonale


use async_trait::async_trait;
use voting_domain::entities::{Vote, VoteCount};

//  VotePublisher
#[async_trait] 
pub trait VotePublisher: Send + Sync {
    async fn enqueue(&self, vote: Vote) -> Result<(), String>;
}

//  VoteConsumer
#[async_trait]
pub trait VoteConsumer: Send + Sync {
    async fn dequeue(&self) -> Result<Option<Vote>, String>;
}

//  VoteRepository
#[async_trait]
pub trait VoteRepository: Send + Sync {
    async fn save_or_update(&self, vote: Vote) -> Result<(), String>;

    async fn count(&self) -> Result<VoteCount, String>;
}
