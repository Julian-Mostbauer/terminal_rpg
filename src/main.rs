mod game_features;
use game_features::character::entity_maker;
use game_features::helper_module::io;

fn main() {
    let item1 = game_features::character::item_maker::new_random_item();
    println!("{:.?}", item1);
    io::fancy_display::show_options();
}
