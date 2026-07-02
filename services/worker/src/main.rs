// ═══════════════════════════════════════════════════
// WORKER — Processus qui traite les votes de Redis → PostgreSQL
// Tourne en boucle infinie
// Consomme Redis (VoteConsumer) et persiste dans PostgreSQL (VoteRepository)
// ═══════════════════════════════════════════════════

use voting_application::ports::{VoteConsumer, VoteRepository};
use voting_infrastructure::redis_adapter::RedisAdapter;
use voting_infrastructure::postgres_adapter::PostgresAdapter;

#[tokio::main]
async fn main() {
    // Lire les URLs depuis les variables d'environnement
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or("redis://127.0.0.1:6379".to_string());
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:postgres@localhost:5432/votes_db".to_string());

    // Créer les adapters
    // RedisAdapter implémente VoteConsumer → lit depuis Redis
    let consumer = RedisAdapter::new(&redis_url);
    // PostgresAdapter implémente VoteRepository → écrit dans PostgreSQL
    let repo = PostgresAdapter::new(&db_url).await;

    println!("✅ Worker démarré — en attente de votes...");

    // Boucle infinie — le worker tourne tant que l'application est lancée
    loop {
        // Tenter de lire un vote depuis Redis
        match consumer.dequeue().await {

            // CAS 1 — Un vote est disponible dans Redis
            Ok(Some(vote)) => {
                println!("📩 Vote reçu → sauvegarde en cours...");

                // Sauvegarder dans PostgreSQL via le PORT VoteRepository
                // ON CONFLICT → si même voter_id, remplace l'ancien vote
                match repo.save_or_update(vote).await {
                    Ok(_) => println!("✅ Vote sauvegardé en base"),
                    Err(e) => println!("❌ Erreur PostgreSQL : {}", e),
                }
            }

            // CAS 2 — File Redis vide, rien à traiter
            Ok(None) => {
                // Attendre 100ms avant de réessayer
                // Évite de consommer 100% du CPU en boucle vide
                tokio::time::sleep(
                    tokio::time::Duration::from_millis(100)
                ).await;
            }

            // CAS 3 — Erreur de connexion Redis
            Err(e) => {
                println!("❌ Erreur Redis : {}", e);
                // Attendre 1 seconde avant de réessayer
                // Laisse le temps à Redis de se reconnecter
                tokio::time::sleep(
                    tokio::time::Duration::from_secs(1)
                ).await;
            }
        }
    }
}
