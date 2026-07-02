

mod templates;

use axum::{
    Router,
    routing::get,
    Json,
    extract::State,
    response::{IntoResponse, Html, sse::{Event, Sse}},
};
use std::sync::Arc;
use std::time::Duration;
use tokio_stream::StreamExt; 
use voting_application::ports::VoteRepository;
use voting_infrastructure::postgres_adapter::PostgresAdapter;

// AppState état partagé entre les handlers
struct AppState {
    repo: Arc<dyn VoteRepository>,
}

// Handler GET / — page HTML des résultat
async fn home() -> Html<String> {
    Html(templates::results_page())
}

// Handler GET /health — health check
async fn health() -> &'static str { "OK" }

// Handler GET /results — retourne les scores en JSON
// Utilisé par le JavaScript pour le chargement initial
async fn get_results(
    State(state): State<Arc<AppState>>
) -> impl IntoResponse {
    match state.repo.count().await {
        Ok(count) => Json(serde_json::json!({
            "cats": count.cats,
            "dogs": count.dogs,
            "total": count.total() // méthode du domaine
        })).into_response(),
        Err(e) => Json(serde_json::json!({"error": e})).into_response(),
    }
}

// Handler GET /results/events 
// le serveur envoie des événements au client
async fn sse_results(
    State(state): State<Arc<AppState>>,
) -> Sse<impl futures::Stream<Item = Result<Event, std::convert::Infallible>>> {

    // Créer un tick toutes les secondes
    let interval = tokio::time::interval(Duration::from_secs(1));

    // Convertir l'interval en Stream (séquence async d'événements)
    let stream = tokio_stream::wrappers::IntervalStream::new(interval);

    // Pour chaque tick lire PostgreSQL  envoyer un événement SSE
    let sse_stream = stream.then(move |_| {
        let repo = state.repo.clone(); // clone Arc (pas les données)
        async move {
            // Lire les scores depuis PostgreSQL
            let data = match repo.count().await {
                Ok(c) => serde_json::json!({
                    "cats": c.cats,
                    "dogs": c.dogs,
                    "total": c.total()
                }),
                // s'il y a une erreur , envoyer des zéros plutôt que crasher
                Err(_) => serde_json::json!({"cats":0,"dogs":0,"total":0}),
            };

            // Créer l'événement SSE nom de l'événement et données JSON envoyées
            Ok(Event::default()
                .event("score")
                .data(data.to_string()))
        }
    });

    // Retourner le flux ssse
    Sse::new(sse_stream)
}

#[tokio::main]
async fn main() {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:postgres@localhost:5432/votes_db".to_string());

    // Créer l'adapter PostgreSQL (implémente VoteRepository)
    let repo = Arc::new(PostgresAdapter::new(&db_url).await);
    let state = Arc::new(AppState { repo });

    let app = Router::new()
        .route("/", get(home))                        // page résultats
        .route("/health", get(health))                // health check
        .route("/results", get(get_results))          // scores JSON
        .route("/results/events", get(sse_results))   // flux SSE live
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081")
        .await.unwrap();
    println!(" result-service démarré sur http://localhost:8081");
    axum::serve(listener, app).await.unwrap();
}
