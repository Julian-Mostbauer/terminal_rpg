mod game_features;
use game_features::character::Entity;
use game_features::character::Item;
use game_features::helper_module::save_n_load;
use game_features::helper_module::save_n_load::load_struct_from_json;

const GOLDSWORD: &str =
    r"C:\Users\julia\OneDrive\Dokumente\GitHub\terminal_rpg\src\assets\items\gold_sword.json";
const BERSERK: &str = r"C:\Users\julia\OneDrive\Dokumente\GitHub\terminal_rpg\src\assets\saved_characters\random_berserker.json";

fn main() {
    let mut berserk: Entity = match load_struct_from_json(BERSERK) {
        Ok(item) => item,
        Err(_e) => game_features::character::entity_maker::new_rand_barbarian(),
    };

    let gold_sword: Item = match load_struct_from_json(GOLDSWORD) {
        Ok(item) => item,
        Err(_e) => game_features::character::item_maker::new_random_item(),
    };

    berserk.obtain_item(&gold_sword);
    println!("{:.?}", berserk);
    berserk.remove_item(&gold_sword);
    println!("{:.?}", berserk);
    save_n_load::save_obj(BERSERK, &berserk);
}
