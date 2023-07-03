use std::{path::Path, fs, io, collections::HashMap};

use mtgjson::{Identifiers, ForeignData, Ruling, Meta};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CardAtomic {
    asciiName: Option<String>,
    attractionLights: Option<Vec<String>>,
    colorIdentity: Vec<String>,
    colorIndicator: Option<Vec<String>>,
    colors: Vec<String>,
    convertedManaCost: f64,
    defense: Option<String>,
    edhrecRank: Option<f64>,
    edhrecSaltiness: Option<f64>,
    faceConvertedManaCost: Option<f64>,
    faceManaValue: Option<f64>,
    faceName: Option<String>,
    firstPrinting: Option<String>,
    foreignData: Option<Vec<ForeignData>>,
    hand: Option<String>,
    hasAlternativeDeckLimit: Option<bool>,
    identifiers: Identifiers,
    isFunny: Option<bool>,
    isReserved: Option<bool>,
    keywords: Option<Vec<String>>,
    layout: String,
    leadershipSkills: Option<mtgjson::LeadershipSkills>,
    legalities: mtgjson::Legalities,
    life: Option<String>,
    loyalty: Option<String>,
    manaCost: Option<String>,
    manaValue: f64,
    name: String,
    power: Option<String>,
    printings: Option<Vec<String>>,
    purchaseUrls: mtgjson::PurchaseUrls,
    relatedCards: Option<mtgjson::RelatedCards>,
    rulings: Option<Vec<Ruling>>,
    side: Option<String>,
    subsets: Option<Vec<String>>,
    subtypes: Vec<String>,
    supertypes: Vec<String>,
    text: Option<String>,
    toughness: Option<String>,
    #[serde(rename = "type")]
    card_type: String,
    types: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cards {
    pub meta: Meta,
    pub data: HashMap<String, Vec<CardAtomic>>,
}


/// Loads the dataset from the given path.
pub fn load_dataset(path: &Path) -> io::Result<Cards> {

    // Read the file.
    let data = fs::read_to_string(path)?;
    let dataset: Cards = serde_json::from_str(&data)?;

    Ok(dataset)
}