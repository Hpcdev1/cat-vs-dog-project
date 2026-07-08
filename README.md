# Cat vs Dog Voting Application

## Présentation

Cat vs Dog est une application de vote développée en Rust permettant aux utilisateurs de choisir entre deux candidats : Cat et Dog.

L'objectif principal du projet est de mettre en oeuvre une architecture découpée en plusieurs services afin de séparer les responsabilités et de rendre l'application plus facilement maintenable et évolutive.

L'interface utilisateur est développée en HTML, CSS et JavaScript, tandis que la partie serveur est développée avec le framework Axum.

Les échanges entre le navigateur et les services Rust sont réalisés à l'aide de requêtes AJAX (`fetch`), ce qui permet d'envoyer ou de récupérer des données sans recharger la page.

---

## Création de la structure du projet

Le projet a été créé sous la forme d'un workspace Rust afin de partager facilement du code entre plusieurs applications.

Les commandes utilisées sont les suivantes :

```bash
# Création du dossier principal
mkdir cat-vs-dog
cd cat-vs-dog

# Création des dossiers
mkdir services
mkdir crates

# Création des services exécutables
cargo new services/vote-service --bin
cargo new services/result-service --bin
cargo new services/worker --bin

# Création des bibliothèques partagées
cargo new crates/voting-domain --lib
cargo new crates/voting-application --lib
cargo new crates/voting-infrastructure --lib
```

Les dossiers créés avec `--bin` possèdent un fichier `main.rs` et représentent des applications exécutables.

Les dossiers créés avec `--lib` sont des bibliothèques destinées à être réutilisées par plusieurs services.

---

## Choix d'architecture

Le projet s'appuie sur deux principes :

- Domain Driven Design (DDD)
- Architecture Hexagonale

### Domain Driven Design (DDD)

Le DDD consiste à placer la logique métier au centre du projet.

Dans cette application, le domaine est représenté principalement par le crate voting-domain, qui contient les concepts importants du système :

- Vote
- Candidate
- VoterId
- VoteCount

Ces structures représentent le fonctionnement du système de vote indépendamment des technologies utilisées.

### Architecture Hexagonale

L'architecture hexagonale consiste à séparer le coeur métier des composants techniques.

Le métier ne dépend pas directement de PostgreSQL, Redis ou des routes HTTP.

Les différents services et les bases de données viennent simplement se connecter autour du domaine grâce à des interfaces bien définies.

Cette organisation facilite les tests, la maintenance et le remplacement d'une technologie par une autre.

---

## Organisation du projet

Le projet est divisé en deux grands dossiers :

### services/

Ce dossier contient les applications exécutables :

- vote-service
- result-service
- worker

### crates/

Ce dossier contient les bibliothèques communes utilisées par les différents services :

- voting-domain
- voting-application
- voting-infrastructure

Cette séparation permet d'éviter la duplication du code et de partager facilement les mêmes structures et les mêmes interfaces.

---

## Description des différents composants

### voting-domain

Ce crate représente le coeur métier de l'application.

Il contient notamment :

- Vote
- Candidate
- VoterId
- VoteCount

Aucune dépendance à PostgreSQL ou Redis n'est présente dans cette partie.

### voting-application

Ce crate contient les cas d'utilisation de l'application.

Il définit notamment les traits permettant :

- d'envoyer un vote
- de lire un vote
- de sauvegarder un vote
- de compter les votes

Ces traits représentent les opérations que l'application sait effectuer.

### voting-infrastructure

Ce crate contient les implémentations techniques des différents traits.

Il permet donc de connecter le domaine métier aux outils techniques utilisés par l'application.

### vote-service

Le vote-service est chargé de recevoir les votes des utilisateurs.

Adresse :

```
http://localhost:8000
```

Routes disponibles :

**GET /**

Retourne la page HTML permettant à l'utilisateur de voter.

**POST /vote**

Reçoit un vote envoyé depuis le navigateur.

Exemple :

```json
{
  "candidate": "cat"
}
```

ou

```json
{
  "candidate": "dog"
}
```

Le service transforme ensuite cette information en objet Vote puis l'envoie dans Redis.

**GET /health**

Permet de vérifier que le service est disponible.

### worker

Le worker fonctionne en arrière-plan.

Il récupère les votes présents dans Redis puis les sauvegarde ou les met à jour dans PostgreSQL.

Cette approche évite d'effectuer directement les écritures dans la base de données lors d'une requête HTTP.

### result-service

Le result-service permet de consulter les résultats du vote.

Adresse :

```
http://localhost:8081
```

Routes disponibles :

**GET /**

Retourne la page HTML affichant les résultats.

Cette page utilise JavaScript pour récupérer les scores via une requête AJAX.

**GET /results**

Retourne les résultats actuels sous forme JSON.

Exemple :

```json
{
  "cats": 10,
  "dogs": 7,
  "total": 17
}
```

**GET /health**

Permet de vérifier que le service fonctionne correctement.

**GET /results/events**

Route prévue pour fournir les résultats en temps réel via les Server-Sent Events (SSE).

Cette fonctionnalité n'a pas été implémentée dans cette version du projet.

---

## Communication entre le Front-end et le Back-end

### Envoi des votes

Lorsque l'utilisateur clique sur un bouton de vote, JavaScript envoie une requête fetch vers :

```
POST /vote
```

Le vote est reçu par le vote-service puis envoyé dans Redis.

Le worker lit ensuite Redis et met à jour PostgreSQL.

### Consultation des résultats

La page des résultats envoie une requête :

```
GET /results
```

Le result-service interroge PostgreSQL et retourne :

```json
{
  "cats": 10,
  "dogs": 7,
  "total": 17
}
```

Le JavaScript met ensuite automatiquement à jour les éléments HTML affichant les scores.

---

## Utilisation de Docker

Le projet était initialement prévu pour fonctionner avec Docker afin d'exécuter automatiquement PostgreSQL et Redis dans des conteneurs.

Cependant, des difficultés techniques rencontrées lors de la configuration ont conduit à utiliser directement des installations locales de PostgreSQL et Redis.

Il est donc nécessaire de disposer de ces deux services sur la machine avant de lancer l'application.

---

## Prérequis

- Rust
- Cargo
- PostgreSQL
- Redis

---

## Lancement du projet

Démarrer Redis :

```bash
redis-server
```

Lancer le service de vote :

```bash
cargo run --bin vote-service
```

Lancer le worker :

```bash
cargo run --bin worker
```

Lancer le service de résultats :

```bash
cargo run --bin result-service
```

---

## Tester l'application

### Voter

Ouvrir :

```
http://localhost:8000
```

Sélectionner Cat ou Dog.

Le vote est envoyé au vote-service puis traité par Redis et le worker avant d'être enregistré dans PostgreSQL.

### Consulter les résultats

Ouvrir :

```
http://localhost:8081
```

La page récupère automatiquement les résultats en appelant :

```
GET /results
```

et affiche les scores retournés par l'API.

---

## Fonctionnalités terminées

- Création d'un workspace Rust
- Mise en place d'une architecture hexagonale
- Organisation du domaine selon les principes du DDD
- Séparation en plusieurs services
- Interface HTML/CSS/JavaScript
- Communication AJAX (fetch)
- Réception des votes via POST /vote
- Utilisation de Redis comme intermédiaire
- Traitement asynchrone des votes par le worker
- Sauvegarde dans PostgreSQL
- Comptage des votes
- API GET /results
- Affichage dynamique des résultats

---

## Fonctionnalités non terminées

- Implémentation de GET /results/events avec les Server-Sent Events (SSE)
- Gestion avancée des erreurs
- Tests automatisés
- Déploiement complet avec Docker