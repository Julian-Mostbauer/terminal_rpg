mod game_features;
use game_features::character_creation;

fn main() {
    let player = character_creation::EntityMaker::make_rand_barbarian();
    println!("{:.?}", player);
}
