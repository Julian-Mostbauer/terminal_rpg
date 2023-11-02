mod game_features;
//use game_features::character_creation;
use game_features::character_creation::EntityMaker;
fn main() {
    let hord = EntityMaker::make_rand_barbarian_hord(100);

    for ent in hord {
        println!("{:.?}", ent);
    }
}
