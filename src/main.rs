mod game_features;
use game_features::character::*;
use game_features::toolbox::*;

use crate::game_features::helper_module::save_n_load;

fn main() {
    let gold_sword: Item = load_preset_item(GOLDSWORD);
    let rand_bersek = game_features::character::entity_maker::new_rand_of_class(Classes::Barbarian);

    let new_item = game_features::character::Item::new(
        "Bow".to_string(),
        ItemTypes::Weapon,
        ItemSubTypes::Axe,
        10,
        10,
        100,
    );
    save_n_load::save_obj("Bow", &new_item);
    println!("{:.?}", gold_sword);
    println!("{:.?}", rand_bersek);
}
