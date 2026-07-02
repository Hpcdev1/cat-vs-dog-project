// ═══════════════════════════════════════════════════
// VOTE-SERVICE — Point d'entrée HTTP pour les votes
// Reçoit les votes via HTTP POST /vote
// Identifie le votant via cookie UUID
// Envoie le vote dans Redis via VotePublisher
// ═══════════════════════════════════════════════════

mod templates; // module templates.rs dans le même dossier

use axum::{
    Router,
    routing::{get, post},
    Json,
    extract::State,
    response::{IntoResponse, Html, Response},
    http::{StatusCode, HeaderMap, header},
};
use axum_extra::extract::CookieJar; // lit les cookies de la requête
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde::Deserialize;
use std::sync::Arc; // Arc = pointeur partagé thread-safe
use voting_domain::entities::{Vote, VoterId, Candidate};
use voting_application::ports::VotePublisher;
use voting_infrastructure::redis_adapter::RedisAdapter;

// AppState — état partagé entre tous les handlers HTTP
// Arc<dyn VotePublisher> = référence partagée vers n'importe quel
// type qui implémente VotePublisher (ici RedisAdapter)
struct AppState {
    publisher: Arc<dyn VotePublisher>,
}

// VoteRequest — structure du JSON reçu en POST /vote
// { "choice": "cat" } ou { "choice": "dog" }
#[derive(Deserialize)]
struct VoteRequest {
    choice: String,
}

// Handler GET / — sert la page HTML de vote
async fn home() -> Html<String> {
    Html(templates::vote_page()) // retourne le HTML depuis templates.rs
}

// Handler GET /health — vérification que le service tourne
async fn health() -> &'static str { "OK" }

// Handler POST /vote — reçoit et traite un vote
async fn post_vote(
    State(state): State<Arc<AppState>>, // accès à l'état partagé
    jar: CookieJar,                     // cookies de la requête HTTP
    Json(payload): Json<VoteRequest>,   // body JSON désérialisé
) -> Response {

    // ÉTAPE 1 — Identifier le votant via cookie
    // Si cookie "voter_id" existe → même personne qui revote
    // Si pas de cookie → nouveau votant, on génère un UUID
    let voter_id = match jar.get("voter_id") {
        Some(cookie) => {
            println!("🔄 Revote pour: {}", cookie.value());
            cookie.value().to_string() // réutilise l'UUID existant
        }
        None => {
            // uuid::Uuid::new_v4() = génère un UUID aléatoire unique
            let new_id = uuid::Uuid::new_v4().to_string();
            println!("🆕 Nouveau votant: {}", new_id);
            new_id
        }
    };

    // ÉTAPE 2 — Valider le choix (règle métier du domaine)
    let candidate = match payload.choice.as_str() {
        "cat" => Candidate::Cat,
        "dog" => Candidate::Dog,
        _ => {
            // Choix invalide → réponse 400 Bad Request
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "Choix invalide : cat ou dog"}))
            ).into_response();
        }
    };

    // ÉTAPE 3 — Créer l'objet Vote (entité du domaine)
    let vote = Vote {
        voter_id: VoterId(voter_id.clone()), // VoterId wraps le String
        candidate,
    };

    // ÉTAPE 4 — Envoyer dans Redis via le PORT VotePublisher
    // On appelle le port, pas directement Redis
    // = le handler ne sait pas que c'est Redis qui est utilisé
    match state.publisher.enqueue(vote).await {
        Ok(_) => {
            // ÉTAPE 5 — Créer/renouveler le cookie voter_id
            // Ce cookie identifie le votant à chaque requête
            let cookie = Cookie::build(("voter_id", voter_id))
                .path("/")           // valide pour tout le site
                .same_site(SameSite::Lax)
                .http_only(false)    // JavaScript peut le lire
                .build();

            // Ajouter le cookie dans les headers de réponse
            let updated_jar = jar.add(cookie);
            let mut headers = HeaderMap::new();
            for c in updated_jar.iter() {
                headers.append(
                    header::SET_COOKIE,
                    c.to_string().parse().unwrap()
                );
            }

            // Réponse 200 OK + cookie + message JSON
            (
                StatusCode::OK,
                headers,
                Json(serde_json::json!({"message": "Vote enregistré"}))
            ).into_response()
        }
        // Erreur Redis → réponse 500
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e}))
        ).into_response(),
    }
}

// Point d'entrée du service
// #[tokio::main] = transforme main en fonction async (nécessaire pour axum)
#[tokio::main]
async fn main() {
    // Lire l'URL Redis depuis la variable d'environnement
    // unwrap_or = valeur par défaut si variable non définie
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or("redis://127.0.0.1:6379".to_string());

    // Créer l'adapter Redis (implémente VotePublisher)
    // Arc::new = mettre dans un pointeur partagé thread-safe
    let publisher = Arc::new(RedisAdapter::new(&redis_url));

    // Créer l'état partagé entre tous les handlers
    let state = Arc::new(AppState { publisher });

    // Définir les routes HTTP
    let app = Router::new()
        .route("/", get(home))          // page de vote
        .route("/health", get(health))  // health check
        .route("/vote", post(post_vote)) // recevoir un vote
        .with_state(state);             // injecter l'état

    // Démarrer le serveur sur le port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await.unwrap();
    println!("✅ vote-service démarré sur http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}
