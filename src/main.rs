mod game_features;
use game_features::character_creation::entity_maker;
use game_features::helper_module::io::read_number;
fn main() {
    let amount: u32 = read_number("Enter Amount: ");
    let new_berserker_hord = entity_maker::make_rand_barbarian_hord(amount);

    for berserk in new_berserker_hord {
        println!("{:.?}", berserk);
    }
}
