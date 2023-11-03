mod game_features;
use game_features::character::entity_maker;
use game_features::helper_module::io;

fn main() {
    let bers1 = entity_maker::make_rand_barbarian();
    println!("{:.?}", bers1);
    io::fancy_display::show_options();
}
