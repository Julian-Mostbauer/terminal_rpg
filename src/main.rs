mod game_features;
use game_features::character;
fn main() {
    let item = character::item_maker::new_random_item();
    //    let berserk = character::entity_maker::make_rand_barbarian();

    println!(
        "{}",
        game_features::helper_module::save_n_load::serialize_struct_to_json(&item).unwrap()
    );
    game_features::helper_module::save_n_load::write_json_to_file(
        "random_item.json",
        &game_features::helper_module::save_n_load::serialize_struct_to_json(&item).unwrap(),
    )
}
