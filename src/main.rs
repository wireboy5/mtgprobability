use std::path::Path;

use mtgprobability::{dataset::Dataset, Deck};



fn main() {

    // Load the dataset
    let dataset = Dataset::load(Path::new("./StandardAtomic.json")).unwrap();


    // Create a new, empty deck
    let mut deck = Deck::new();

    let sheoldred = dataset.get("Sheoldred, the Apocalypse").unwrap();
    println!("{:#?}", sheoldred);
}
