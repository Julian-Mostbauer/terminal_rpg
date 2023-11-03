mod game_features;
use game_features::helper_module::io;

fn main() {
    let item1 = game_features::character::item_maker::gen_item_from_file("iron_sword");

    println!("{:.?}", item1);
    io::fancy_display::show_options();
}
