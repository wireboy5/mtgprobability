use std::{path::Path, fs, io, collections::HashMap};

use mtgjson::{Identifiers, ForeignData, Ruling, Meta};
use serde::{Serialize, Deserialize};

use crate::{Card, ManaColor};



#[derive(Serialize, Deserialize, Debug)]
struct CardAtomic {
    asciiName: Option<String>,
    attractionLights: Option<Vec<String>>,
    colorIdentity: Vec<String>,
    colorIndicator: Option<Vec<String>>,
    colors: Vec<String>,
    convertedManaCost: Option<f64>,
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
    manaValue: Option<f64>,
    name: String,
    power: Option<String>,
    printings: Option<Vec<String>>,
    purchaseUrls: Option<mtgjson::PurchaseUrls>,
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
struct Cards {
    pub meta: Meta,
    pub data: HashMap<String, Vec<CardAtomic>>,
}


/// A dataset of all MTG cards.
pub struct Dataset {
    data: Cards,
}

impl Dataset {
    /// Loads the dataset from the given path.
    pub fn load(path: &Path) -> io::Result<Dataset> {
        // Read the file.
        let data = fs::read_to_string(path)?;
        let data: Cards = serde_json::from_str(&data)?;

        Ok(Dataset { data })
    }

    /// Get a Card by its name.
    pub fn get(&self, name: &str) -> Option<Card> {
        // Load the card's raw data
        let data = self.data.data.get(name)?;

        // Just get the first one
        let card = data.first()?;
        
        // Create the card
        let card = Card::Card {
            name: card.name.clone(),
            mana_cost: {
                // Parse the mana cost as a list of braced values
                let mut stack = String::new();
                let mut parsed = Vec::<String>::new();

                for v in card.manaCost.clone().unwrap_or("".to_string()).chars() {
                    // If this is a opening brace, reset the stack
                    if v == '{' {
                        stack.clear();
                    } else if v == '}' {
                        // If a closing brace, put the stack onto parsed
                        parsed.push(stack.clone());
                    } else {
                        // Otherwise, push the value onto the stack
                        stack.push(v);
                    }
                }

                // Map the parsed values into a Vec<ManaCost>
                parsed.into_iter()
                    .filter_map(|v| {
                        match v.as_str() {
                            "W" => Some(vec![ManaColor::White]),
                            "U" => Some(vec![ManaColor::Blue]),
                            "B" => Some(vec![ManaColor::Black]),
                            "R" => Some(vec![ManaColor::Red]),
                            "G" => Some(vec![ManaColor::Green]),
                            other => {
                                // If we can parse into a u8, use it
                                match other.parse::<u8>() {
                                    Ok(v) => Some(vec![ManaColor::Colorless; v as usize]),
                                    Err(_) => None,
                                }
                            }
                        }
                    })
                    .flatten()
                    .collect()
            },
            text: card.text.clone().unwrap_or("".to_string())
        };
        
        Some(card)
    }
}



