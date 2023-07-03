use std::path::Path;

fn main() {

    // Load the MTG dataset
    let dataset = mtgprobability::load_dataset(Path::new("StandardAtomic.json")).unwrap();
    println!("{:?}", dataset.data.get("Sheoldred, the Apocalypse").unwrap());
}
