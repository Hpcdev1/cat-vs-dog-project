use async_trait::async_trait;
use sqlx::PgPool;
use voting_domain::entities::{Vote, VoteCount, Candidate};
use voting_application::ports::VoteRepository;

pub struct PostgresAdapter {
    pool: PgPool,
}

impl PostgresAdapter {
    pub async fn new(url: &str) -> Self {
        let pool = PgPool::connect(url).await
            .expect("Impossible de connecter à PostgreSQL");
        PostgresAdapter { pool }
    }
}

#[async_trait]
impl VoteRepository for PostgresAdapter {
    async fn save_or_update(&self, vote: Vote) -> Result<(), String> {
        let choice = match vote.candidate {
            Candidate::Cat => "cat",
            Candidate::Dog => "dog",
        };

        sqlx::query(
            r#"
            INSERT INTO votes (voter_id, choice, updated_at)
            VALUES ($1, $2, now())
            ON CONFLICT (voter_id)
            DO UPDATE SET choice = $2, updated_at = now()
            "#,
        )
        .bind(&vote.voter_id.0)
        .bind(choice)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn count(&self) -> Result<VoteCount, String> {
        let cats: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM votes WHERE choice = 'cat'"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let dogs: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM votes WHERE choice = 'dog'"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(VoteCount {
            cats: cats as u64,
            dogs: dogs as u64,
        })
    }
}
