mod game_features;
use game_features::character::*;
use game_features::toolbox::*;

fn main() {
    let gold_sword: Item = load_preset_item(GOLDSWORD);
    let rand_bersek = game_features::character::entity_maker::new_rand_of_class(Classes::Barbarian);

    println!("{:.?}", gold_sword);
    println!("{:.?}", rand_bersek);
}
