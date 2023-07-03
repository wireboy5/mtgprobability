use std::path::Path;

use mtgprobability::{Deck, dataset::Dataset, ManaColor};
fn main() {

    // Load the dataset
    let dataset = Dataset::load(Path::new("./StandardAtomic.json")).unwrap();


    // Create a new, empty deck
    let mut deck = Deck::new();

    // Legendary cards
    deck.add_cards(4, dataset.get("Sheoldred, the Apocalypse").unwrap());
    deck.add_cards(4, dataset.get("Liesa, Forgotten Archangel").unwrap());
    deck.add_cards(2, dataset.get("Elas il-Kor, Sadistic Pilgrim").unwrap());

    // Creatures
    deck.add_cards(4, dataset.get("Fell Stinger").unwrap());
    deck.add_cards(4, dataset.get("Mindleech Ghoul").unwrap());
    deck.add_cards(4, dataset.get("Inspiring Overseer").unwrap());
    deck.add_cards(4, dataset.get("Spirited Companion").unwrap());
    deck.add_cards(4, dataset.get("Morbid Opportunist").unwrap());
    deck.add_cards(2, dataset.get("Corrupt Court Official").unwrap());

    // Sorceries
    deck.add_cards(2, dataset.get("Soul Transfer").unwrap());

    // Instants
    deck.add_cards(4, dataset.get("Hero's Downfall").unwrap());

    // Get the land counts needed for four out of seven cards to be lands
    let land_counts = deck.lands_needed(4.0/7.0);
    println!("{:?}", land_counts);

    // Add the land counts to the deck
    deck.add_lands(land_counts);

    println!("{}", deck.as_mtga());
}