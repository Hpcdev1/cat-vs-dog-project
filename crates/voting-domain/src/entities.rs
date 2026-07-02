
// le domaine métier  

use serde::{Serialize, Deserialize};

//  les deux choix 
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Candidate {
    Cat, //  représente le chat
    Dog, // représente   le chien
}

// VoterId  identifiant unique du votant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoterId(pub String);

// gestion de qui a voté et pour qui
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub voter_id: VoterId,    
    pub candidate: Candidate, 
}

// compter le nombre de vote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteCount {
    pub cats: u64, 
    pub dogs: u64, 
}

impl VoteCount {
    // Calcule le nombre de vote
    pub fn total(&self) -> u64 {
        self.cats + self.dogs
    }
}
