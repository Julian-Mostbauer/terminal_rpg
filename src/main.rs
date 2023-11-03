mod game_features;
use game_features::character_creation::EntityMaker;
use game_features::helper_module::read_number;
fn main() {
    let amount: f32 = read_number("Enter Amount: ");
    println!("You entered {amount}");
}
