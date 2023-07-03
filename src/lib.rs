use std::{collections::HashMap, fmt::Display};

use rand::prelude::IteratorRandom;

pub mod dataset;

/// Structure containing the probability of each land type
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LandProbabilities {
    pub swamp: f64,
    pub plain: f64,
    pub forest: f64,
    pub mountain: f64,
    pub island: f64,
}



impl LandProbabilities {
    /// Normalizes the land probabilities to have a total of 1
    pub fn normalize(mut self) -> Self {
        // Sum the probabilities
        let sum: f64 = self.swamp + self.plain + self.forest + self.mountain + self.island;

        // Normalize the probabilities
        self.swamp /= sum;
        self.plain /= sum;
        self.forest /= sum;
        self.mountain /= sum;
        self.island /= sum;

        self
    }

    /// Normalizes the land probabilities and splits a value among them, adding the split value to the probabilities
    pub fn divy_normalized(self, value: f64) -> Self {
        // Get the normalized value
        let normalized = self.normalize();

        // Split the value among the probabilities
        Self {
            swamp: self.swamp + normalized.swamp * value,
            plain: self.plain + normalized.plain * value,
            forest: self.forest + normalized.forest * value,
            mountain: self.mountain + normalized.mountain * value,
            island: self.island + normalized.island * value,
        }
    }

    /// From a number of total lands, gets the number of each land needed, roudning up
    pub fn needed(&self, total: f64) -> LandsNeeded {
        LandsNeeded {
            swamp: (self.swamp * total).ceil() as usize,
            plain: (self.plain * total).ceil() as usize,
            forest: (self.forest * total).ceil() as usize,
            mountain: (self.mountain * total).ceil() as usize,
            island: (self.island * total).ceil() as usize,
        }
    }
}

/// Structure containing the number of each land type needed in a deck
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LandsNeeded {
    pub swamp: usize,
    pub plain: usize,
    pub forest: usize,
    pub mountain: usize,
    pub island: usize,
}

/// A manna color
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ManaColor {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}

/// A card
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Card {
    // A land
    Land(ManaColor),
    // A card
    Card {
        /// The name of the card
        name: String,
        /// The mana cost of the card
        mana_cost: Vec<ManaColor>,
        /// The text of the card
        text: String,
    },
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Card::Card { name, ..} => name,
            Card::Land(color) => match color {
                ManaColor::White => "Plains",
                ManaColor::Blue => "Island",
                ManaColor::Black => "Swamp",
                ManaColor::Red => "Mountain",
                ManaColor::Green => "Forest",
                ManaColor::Colorless => "",
            }
        })
    }
}

/// Represents a deck of cards
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        Deck {
            cards: Vec::new(),
        }
    }


    /// The number of cards in the deck
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Add a card to the deck
    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Add N of a card to the deck
    pub fn add_cards(&mut self, n: usize, card: Card) {
        for _ in 0..n {
            self.cards.push(card.clone());
        }
    }

    /// Adds lands to the deck according to the land counts
    pub fn add_lands(&mut self, land_counts: LandsNeeded) {
        
        self.cards.extend(vec![Card::Land(ManaColor::Black); land_counts.swamp]);
        self.cards.extend(vec![Card::Land(ManaColor::White); land_counts.plain]);
        self.cards.extend(vec![Card::Land(ManaColor::Green); land_counts.forest]);
        self.cards.extend(vec![Card::Land(ManaColor::Red); land_counts.mountain]);
        self.cards.extend(vec![Card::Land(ManaColor::Blue); land_counts.island]);
    }

    /// The amount of a specific color of mana in the deck
    pub fn mana_count(&self, color: ManaColor) -> usize {
        self.cards.iter()
            .filter_map(|v| if let Card::Card { mana_cost,.. } = v {
                Some(mana_cost.iter().filter(|v| **v == color).count())
            } else {
                None
            })
            .fold(0, |a,b| a+b)
    }

    /// The number of cards that need a particular color of mana
    pub fn mana_needed(&self, color: ManaColor) -> usize {
        self.cards.iter()
            .filter(|v| {
                if let Card::Card { mana_cost,.. } = v {
                    mana_cost.contains(&color)
                } else {
                    false
                }
            })
            .count()
    }

    /// Find the probability of needing a certain color of Mana to play a card
    pub fn mana_requirement_probability(&self, color: ManaColor) -> f64 {
        // Get the total number of cards in the deck that are not lands
        let card_count = self.card_count() as f64;

        // Get the total number of cards that use that color of Mana
        let color_count = self.mana_needed(color) as f64;
        
        // Calculate the probability of drawing a given color of mana
        color_count / card_count
    }

    // Returns the probability that, given a color of Mana, selecting a random mana value from the deck will satisfy the color
    pub fn color_probability(&self, color: ManaColor) -> f64 {
        
        // Count every mana value in the deck (how much mana it would cost to play the whole deck at once)
        let mana_count = self.cards.iter()
            .filter_map(|v| if let Card::Card { mana_cost,.. } = v {
                Some(mana_cost.len() as f64)
            } else {
                None
            })
            .fold(0.0, |a,b| a+b);
        
        // Count every mana value of the color type in the deck
        let color_count = self.mana_count(color) as f64;

        // Return the probability
        color_count / mana_count
    }

    /// Returns the number of non-land cards in the deck
    pub fn card_count(&self) -> usize {
        self.cards.iter()
            .filter(|v| !matches!(v, Card::Land(_)))
            .count()
    }

    /// Returns the number of land cards in the deck
    pub fn land_count(&self) -> usize {
        self.cards.iter()
            .filter(|v| matches!(v, Card::Land(_)))
            .count()
    }

    /// Finds what percent of lands each land type needs to be.
    pub fn land_probabilities(&self) -> LandProbabilities {
        // Find the what percent of mana values each color of land is
        let land_probabilities = LandProbabilities {
            swamp: self.color_probability(ManaColor::Black),
            plain: self.color_probability(ManaColor::White),
            forest: self.color_probability(ManaColor::Green),
            mountain: self.color_probability(ManaColor::Red),
            island: self.color_probability(ManaColor::Blue),
        };

        // Find what percent of mana values are colorless
        let colorless = self.color_probability(ManaColor::Colorless);

        // Split the colorless probability into the other probabilities, based on the normalized probabilities of each
        let land_probabilities = land_probabilities.divy_normalized(colorless);

        land_probabilities
    }

    /// Find how many lands will be needed, given the target probability of drawing a land.
    /// Note: This returns an f64, and it may not be a whole number.
    pub fn total_lands_needed(&self, prob: f64) -> f64 {
        // The equation for how many lands will be needed is as follows, where l is the number of lands
        // needed, p is the target probability of drawing a land, and c is the number of non-land cards in the deck.
        // p = l/(l+c)
        // l = p * (l + c)
        // l = pl + pc
        // l - pl = pc
        // (1-p)l = pc
        // l = (pc)/(1-p)

        (prob * self.card_count() as f64)/(1.0 - prob)
    }

    /// Finds the most of a single type of mana needed to play a card
    pub fn most_mana_needed(&self, color: ManaColor) -> usize {
        self.cards.iter()
            .map(|v| {
                if let Card::Card { mana_cost,.. } = v {
                    mana_cost.iter()
                        .filter(|v| **v == color)
                        .count()
                } else {
                    0
                }
            })
            .max().unwrap_or(0)
    }
    

    /// Finds how many of each land will be needed
    pub fn lands_needed(&self, prob: f64) -> LandsNeeded {
        // Get the land probabilities
        let land_probabilities = self.land_probabilities();

        // Get the lands needed
        let l = land_probabilities.needed(self.total_lands_needed(prob));

        // Max on the lands needed to make sure it is not impossible to play a card
        LandsNeeded {
            swamp: l.swamp.max(self.most_mana_needed(ManaColor::Black)),
            plain: l.plain.max(self.most_mana_needed(ManaColor::White)),
            forest: l.forest.max(self.most_mana_needed(ManaColor::Green)),
            mountain: l.mountain.max(self.most_mana_needed(ManaColor::Red)),
            island: l.island.max(self.most_mana_needed(ManaColor::Blue)),
        }
    }

    /// Returns X random cards
    pub fn draw(&self, x: usize) -> Vec<Card> {
        self.cards.iter().choose_multiple(&mut rand::thread_rng(), x)
            .into_iter()
            .map(|v| v.clone())
            .collect()
    }

    /// Returns the count of each card in the deck
    pub fn card_counts(&self) -> HashMap<Card, usize> {
        let mut out = HashMap::new();
        for card in self.cards.iter() {
            *out.entry(card.clone()).or_insert(0) += 1;
        }
        out
    }

    /// Exports in MTGA format
    pub fn as_mtga(&self) -> String {
        let mut out = String::new();
        out.push_str("Deck\n");
        let cards = self.card_counts();
        for (card, count) in cards.iter() {
            out.push_str(&format!("{} {} () 0\n", count, card));
        }
        out
    }
}