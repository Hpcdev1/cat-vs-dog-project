

use async_trait::async_trait;
use voting_domain::entities::Vote;
use voting_application::ports::{VotePublisher, VoteConsumer};

// redisadapter contient l'url de connexion a redis

pub struct RedisAdapter {
    url: String, 
}

impl RedisAdapter {
    //creéation d'un adapter a l'url
    pub fn new(url: &str) -> Self {
        RedisAdapter { url: url.to_string() }
    }
}

// Implémentation du PORT VotePublisher pour RedisAdapter
#[async_trait]
impl VotePublisher for RedisAdapter {
    async fn enqueue(&self, vote: Vote) -> Result<(), String> {
        let client = redis::Client::open(self.url.as_str())
            .map_err(|e| e.to_string())?; 

        // ouvrir une connexion async
        let mut conn = client.get_async_connection().await
            .map_err(|e| e.to_string())?;

        // 3. Convertir le Vote en JSON string pour redis avec serde_json
        let json = serde_json::to_string(&vote)
            .map_err(|e| e.to_string())?;


        redis::AsyncCommands::lpush::<_, _, ()>(&mut conn, "votes", json)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}


// lecture des votes depuis redis avec worker
#[async_trait]
impl VoteConsumer for RedisAdapter {
    async fn dequeue(&self) -> Result<Option<Vote>, String> {
        //  Création du client et la connexion
        let client = redis::Client::open(self.url.as_str())
            .map_err(|e| e.to_string())?;
        let mut conn = client.get_async_connection().await
            .map_err(|e| e.to_string())?;

        // Attend maximum 1 seconde qu'un message arrive dans "votes"
        
        let result: Option<(String, String)> =
            redis::AsyncCommands::brpop(&mut conn, "votes", 1.0)
                .await
                .map_err(|e| e.to_string())?;

        // 3. Traiter le résultat
        match result {
            Some((_, val)) => {
                // désérialiser le JSON
                let vote: Vote = serde_json::from_str(&val)
                    .map_err(|e| e.to_string())?;
                Ok(Some(vote)) // vote reçu !
            }
            None => Ok(None), //erreur
        }
    }
}
