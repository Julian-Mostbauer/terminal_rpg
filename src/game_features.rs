/* Refrences, Todos, Infos
    Todo:
    -   Implement random names for everything

    Refrences:
    -   https://donjon.bin.sh/fantasy/name/#type=set
    -   https://reintech.io/blog/working-with-json-in-rust
    -   https://github.com/ratatui-org/ratatui

*/
#![allow(dead_code)]
// -----------------------------------------------------------------------------------------
// CHARACTER CREATION MODULE
// -----------------------------------------------------------------------------------------
pub mod character {
    use rand::distributions::{Distribution, Standard};
    use rand::Rng;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;
    /* Races --------------------------------------------------------------------------------------*/
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum Races {
        ErrorRace,
        Human,
        Elf,
        Dragonborn,
        Dwarf,
        Gnome,
        HalfElf,
        Halfling,
        Tiefling,
        HalfOrc,
    }

    impl Distribution<Races> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Races {
            match rng.gen_range(0..=8) {
                0 => Races::Human,
                1 => Races::Elf,
                2 => Races::Dragonborn,
                3 => Races::Dwarf,
                4 => Races::Gnome,
                5 => Races::HalfElf,
                6 => Races::Halfling,
                7 => Races::Tiefling,
                8 => Races::HalfOrc,
                _ => Races::ErrorRace,
            }
        }
    }

    /* Classes --------------------------------------------------------------------------------------*/
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum Classes {
        ErrorClass,
        Barbarian,
        Bard,
        Cleric,
        Druid,
        Fighter,
        Monk,
        Paladin,
        Ranger,
        Rogue,
        Sorcerer,
        Warlock,
        Wizard,
        Artificer,
    }

    impl Distribution<Classes> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Classes {
            match rng.gen_range(0..=12) {
                0 => Classes::Barbarian,
                1 => Classes::Bard,
                2 => Classes::Cleric,
                3 => Classes::Druid,
                4 => Classes::Fighter,
                5 => Classes::Monk,
                6 => Classes::Paladin,
                7 => Classes::Ranger,
                8 => Classes::Rogue,
                9 => Classes::Sorcerer,
                10 => Classes::Warlock,
                11 => Classes::Wizard,
                12 => Classes::Artificer,
                _ => Classes::ErrorClass,
            }
        }
    }

    /* Abilities --------------------------------------------------------------------------------------*/
    #[derive(Debug, Deserialize, Serialize)]
    pub enum Abilities {
        ErrorAbility,
        Rage,
    }

    impl Distribution<Abilities> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Abilities {
            match rng.gen_range(0..=0) {
                0 => Abilities::Rage,
                _ => Abilities::ErrorAbility,
            }
        }
    }

    /* Spells --------------------------------------------------------------------------------------*/
    #[derive(Debug, Deserialize, Serialize)]
    pub enum Spells {
        ErrorSpell,
        Fireball,
    }

    impl Distribution<Spells> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Spells {
            match rng.gen_range(0..=0) {
                0 => Spells::Fireball,
                _ => Spells::ErrorSpell,
            }
        }
    }
    /* Entity struct --------------------------------------------------------------------------------------*/
    pub type EntityTuple = (
        String,
        Races,
        Classes,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        Vec<Abilities>,
        Vec<Spells>,
        Inventory,
    );
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Entity {
        name: String,
        id: u128,

        race: Races,
        class: Classes,
        lvl: u32,

        strength: u32,
        dexterity: u32,
        constitution: u32,
        intelligence: u32,
        wisdom: u32,
        charisma: u32,

        spell_points: u32,

        abilities: Vec<Abilities>,
        spells: Vec<Spells>,

        pub inventory: Inventory,
    }
    impl Entity {
        pub fn new(parameters: EntityTuple) -> Entity {
            Entity {
                name: parameters.0.to_string(),
                id: Uuid::new_v4().as_u128(),
                race: parameters.1,
                class: parameters.2,
                lvl: parameters.3,
                strength: parameters.4,
                dexterity: parameters.5,
                constitution: parameters.6,
                intelligence: parameters.7,
                wisdom: parameters.8,
                charisma: parameters.9,
                spell_points: parameters.10,
                abilities: parameters.11,
                spells: parameters.12,
                inventory: parameters.13,
            }
        }

        pub fn obtain_item(&mut self, item: &Item) {
            self.inventory.add(item);
        }

        pub fn remove_item(&mut self, item: &Item) {
            self.inventory.remove_by_id(item.id);
        }
    }
    /* Inventory struct --------------------------------------------------------------------------------------*/
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Inventory {
        pub items: Vec<Item>,
        weight: u32,
    }
    impl Inventory {
        pub fn new() -> Inventory {
            Inventory {
                items: Vec::new(),
                weight: 0,
            }
        }
        pub fn add(&mut self, item: &Item) {
            self.items.push(item.clone());
        }
        pub fn remove_by_id(&mut self, id: u128) {
            self.items.retain(|item| item.id != id);
        }
    }

    /* Item Types --------------------------------------------------------------------------------------*/
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum ItemTypes {
        ErrorItemType,
        Weapon,
        Tool,
        Utensil,
    }
    impl Distribution<ItemTypes> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ItemTypes {
            match rng.gen_range(0..=2) {
                0 => ItemTypes::Weapon,
                1 => ItemTypes::Weapon,
                2 => ItemTypes::Utensil,
                _ => ItemTypes::ErrorItemType,
            }
        }
    }
    /* Item Sub Types --------------------------------------------------------------------------------------*/
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum ItemSubTypes {
        ErrorItemSubType,
        Axe,
        Sword,
        Shield,
        Hamer,
        Food,
    }
    impl Distribution<ItemSubTypes> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ItemSubTypes {
            match rng.gen_range(0..=3) {
                0 => ItemSubTypes::Axe,
                1 => ItemSubTypes::Sword,
                2 => ItemSubTypes::Shield,
                3 => ItemSubTypes::Hamer,
                _ => ItemSubTypes::ErrorItemSubType,
            }
        }
    }
    /* Item struct --------------------------------------------------------------------------------------*/
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Item {
        name: String,
        id: u128,
        item_type: ItemTypes,
        item_sub_type: ItemSubTypes,

        weight: u32,
        damage: u32,
        cost: u32,
    }
    impl Item {
        pub fn new(
            name_param: String,
            type_param: ItemTypes,
            sub_type_param: ItemSubTypes,
            weight_param: u32,
            damage_param: u32,
            cost_param: u32,
        ) -> Item {
            Item {
                name: name_param,
                id: Uuid::new_v4().as_u128(),
                item_type: type_param,
                item_sub_type: sub_type_param,
                weight: weight_param,
                damage: damage_param,
                cost: cost_param,
            }
        }

        pub fn get_id(&self) -> u128 {
            self.id
        }
    }
    // -----------------------------------------------------------------------------------------
    // Item Maker MODULE
    // -----------------------------------------------------------------------------------------
    pub mod item_maker {
        use crate::game_features::character::*;
        use crate::game_features::dice;
        use crate::game_features::helper_module;
        use crate::game_features::helper_module::save_n_load;

        pub fn new_random_item() -> Item {
            Item::new(
                helper_module::str!(helper_module::random_names::random_item_name()),
                rand::random(),
                rand::random(),
                dice::d20(),
                dice::d20(),
                dice::d100(),
            )
        }

        pub fn gen_item_from_file(file_name: &str) -> Item {
            save_n_load::load_struct_from_json(file_name).unwrap()
        }

        pub fn gen_whole_folder() -> Vec<Item> {
            use std::fs;

            let paths = fs::read_dir(
                "C:\\Users\\julia\\OneDrive\\Dokumente\\GitHub\\terminal_rpg\\src\\assets\\items",
            )
            .unwrap();

            let mut items: Vec<Item> = Vec::new();

            for file in paths {
                let file_name = helper_module::str!(file.unwrap().path().display());
                println!("{file_name}");
                items.push(gen_item_from_file(&file_name));
            }
            items
        }
    }

    // -----------------------------------------------------------------------------------------
    // Entity Maker MODULE
    // -----------------------------------------------------------------------------------------
    pub mod entity_maker {
        use crate::game_features::character::*;
        use crate::game_features::dice;
        use crate::game_features::helper_module::random_names::random_entity_name;

        pub fn new_rand_hord_of_class(class: Classes, count: u32) -> Vec<Entity> {
            let mut hord: Vec<Entity> = Vec::new();
            for _i in 0..count {
                hord.push(new_rand_of_class(class.clone()));
            }
            hord
        }

        pub fn new_rand_of_class(class: Classes) -> Entity {
            let barb_abilities: Vec<Abilities> = Vec::new();
            let barb_inventory: Inventory = Inventory {
                items: Vec::new(),
                weight: 0,
            };
            let random_race: Races = rand::random();

            let barb_spells = Vec::new();
            let parameters: EntityTuple = (
                random_entity_name(),
                random_race,
                class,
                dice::d20(),
                dice::d20(),
                dice::d20(),
                dice::d20(),
                dice::d20(),
                dice::d20(),
                dice::d20(),
                0,
                barb_abilities,
                barb_spells,
                barb_inventory,
            );
            Entity::new(parameters)
        }
        pub fn new_rand_hord_of_race(race: Races, count: u32) -> Vec<Entity> {
            let mut hord: Vec<Entity> = Vec::new();
            for _i in 0..count {
                hord.push(new_rand_of_race(race.clone()));
            }
            hord
        }

        pub fn new_rand_of_race(race: Races) -> Entity {
            let barb_abilities: Vec<Abilities> = Vec::new();
            let barb_inventory: Inventory = Inventory {
                items: Vec::new(),
                weight: 0,
            };
            let random_class: Classes = rand::random();

            let barb_spells = Vec::new();
            let parameters: EntityTuple = (
                random_entity_name(),
                race,
                random_class,
                dice::d20(),
                dice::d20(),
                dice::d20(),
                dice::d20(),
                dice::d20(),
                dice::d20(),
                dice::d20(),
                0,
                barb_abilities,
                barb_spells,
                barb_inventory,
            );
            Entity::new(parameters)
        }
    }
}

// -----------------------------------------------------------------------------------------
// DICE MODULE
// -----------------------------------------------------------------------------------------
pub mod dice {
    use rand::Rng;

    pub fn d4() -> u32 {
        dn(4)
    }
    pub fn d6() -> u32 {
        dn(6)
    }
    pub fn d8() -> u32 {
        dn(8)
    }
    pub fn d12() -> u32 {
        dn(12)
    }
    pub fn d20() -> u32 {
        dn(20)
    }
    pub fn d100() -> u32 {
        dn(100)
    }
    pub fn dn(n: u32) -> u32 {
        rand::thread_rng().gen_range(1..=n)
    }
}

// -----------------------------------------------------------------------------------------
// HELPER MODULE
// -----------------------------------------------------------------------------------------
pub mod helper_module {
    /* Macros --------------------------------------------------------------------------------------*/
    macro_rules! str {
        ($a: expr) => {
            $a.to_string().trim().to_string()
        };
    }
    pub use str;

    pub mod random_names {
        use crate::game_features::dice;

        pub fn random_entity_name() -> String {
            let possible_names: Vec<String> = vec![
                str!("Alden"),
                str!("Alec"),
                str!("Anton"),
                str!("Arden"),
                str!("Arlen"),
                str!("Armand"),
                str!("Arron"),
                str!("Augustus"),
                str!("Avery"),
                str!("Benedict"),
                str!("Bennett"),
                str!("Branden"),
                str!("Brendon"),
                str!("Britt"),
                str!("Broderick"),
                str!("Carter"),
                str!("Chadwick"),
                str!("Chas"),
                str!("Chet"),
                str!("Colby"),
                str!("Cordell"),
                str!("Dalton"),
                str!("Damien"),
                str!("Dante"),
                str!("Darell"),
                str!("Darius"),
                str!("Darron"),
                str!("Darwin"),
                str!("Dewitt"),
                str!("Diego"),
                str!("Dillon"),
                str!("Dirk"),
                str!("Domenic"),
                str!("Donovan"),
                str!("Dorian"),
                str!("Dorsey"),
                str!("Edison"),
                str!("Elden"),
                str!("Elvin"),
                str!("Erich"),
                str!("Galen"),
                str!("Garret"),
                str!("Gaston"),
                str!("Gavin"),
                str!("German"),
                str!("Graham"),
                str!("Hal"),
                str!("Hank"),
                str!("Harlan"),
                str!("Hayden"),
                str!("Herschel"),
                str!("Hoyt"),
                str!("Hunter"),
                str!("Isaias"),
                str!("Issac"),
                str!("Jacinto"),
                str!("Jarred"),
                str!("Jonas"),
                str!("Kendrick"),
                str!("Keneth"),
                str!("Kennith"),
                str!("Keven"),
                str!("Leif"),
                str!("Lenard"),
                str!("Lincoln"),
                str!("Linwood"),
                str!("Lucius"),
                str!("Lynwood"),
                str!("Malcolm"),
                str!("Malik"),
                str!("Maxwell"),
                str!("McKinley"),
                str!("Merlin"),
                str!("Merrill"),
                str!("Michal"),
                str!("Monty"),
                str!("Newton"),
                str!("Nolan"),
                str!("Porter"),
                str!("Quinton"),
                str!("Raphael"),
                str!("Reid"),
                str!("Rory"),
                str!("Scotty"),
                str!("Shad"),
                str!("Stanton"),
                str!("Stefan"),
                str!("Thaddeus"),
                str!("Tobias"),
                str!("Trenton"),
                str!("Vance"),
                str!("Walker"),
                str!("Walton"),
                str!("Weldon"),
                str!("Wes"),
                str!("Weston"),
                str!("Willian"),
                str!("Winford"),
                str!("Wyatt"),
            ];
            possible_names[dice::dn(possible_names.len() as u32 - 1) as usize].clone()
        }

        pub fn random_item_name() -> String {
            let object_pre: Vec<String> =
                vec![str!("Mighty"), str!("Evil"), str!("Godly"), str!("Fierce")];
            let object_names: Vec<String> = vec![
                str!("Road"),
                str!("Sword"),
                str!("Axe"),
                str!("Bow"),
                str!("Shield"),
                str!("Potion"),
                str!("Ring"),
                str!("Staff"),
                str!("Scroll"),
                str!("Dagger"),
                str!("Helmet"),
                str!("Amulet"),
                str!("Gauntlet"),
                str!("Cloak"),
                str!("Quiver"),
                str!("Wand"),
                str!("Tome"),
                str!("Bracelet"),
                str!("Crown"),
                str!("Robe"),
            ];
            format!(
                "{} {}",
                object_pre[dice::dn(object_pre.len() as u32 - 1) as usize],
                object_names[dice::dn(object_names.len() as u32 - 1) as usize],
            )
        }
    }
    // -----------------------------------------------------------------------------------------
    // IO MODULE
    // -----------------------------------------------------------------------------------------
    pub mod io {
        use num_traits::Num;

        use crate::game_features::helper_module;

        pub enum Colors {
            Red,
            Green,
            Yellow,
            Blue,
            Magenta,
            Cyan,
            White,
        }

        pub fn make_colored(text: &str, color: Colors) -> String {
            match color {
                Colors::Red => format!("\x1b[31m{}\x1b[0m", text),
                Colors::Green => format!("\x1b[32m{}\x1b[0m", text),
                Colors::Yellow => format!("\x1b[33m{}\x1b[0m", text),
                Colors::Blue => format!("\x1b[34m{}\x1b[0m", text),
                Colors::Magenta => format!("\x1b[35m{}\x1b[0m", text),
                Colors::Cyan => format!("\x1b[36m{}\x1b[0m", text),
                Colors::White => format!("\x1b[37m{}\x1b[0m", text),
            }
        }

        pub fn reapeat_str(text: &str, amount: usize) -> String {
            let char_collection: Vec<&str> = vec![text; amount];
            char_collection.concat()
        }

        pub fn read_string(prompt: &str) -> String {
            println!("{prompt}");
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            str!(buf)
        }

        pub fn read_number<T>(prompt: &str) -> T
        where
            T: Num + std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug,
        {
            let mut input = read_string(prompt);
            loop {
                match input.parse::<T>() {
                    Ok(_t) => return input.parse::<T>().unwrap(),
                    Err(_t) => {
                        println!(
                            "{}",
                            make_colored("INVALID", helper_module::io::Colors::Red)
                        );
                        input = read_string(prompt);
                    }
                }
            }
        }

        pub mod fancy_display {
            use crate::game_features::helper_module::io::{self, reapeat_str};

            pub fn show_options() {
                println!(
                    "{}",
                    io::make_colored(&reapeat_str("-", 50), io::Colors::Red)
                );
                println!(" todo ");
                println!(
                    "{}",
                    io::make_colored(&reapeat_str("-", 50), io::Colors::Red)
                );
            }
        }
    }
    // -----------------------------------------------------------------------------------------
    // SAVE N LOAD MODULE
    // -----------------------------------------------------------------------------------------
    pub mod save_n_load {
        use std::error::Error;
        use std::fs;

        use crate::game_features::helper_module;

        pub fn read_file(path: &str) -> String {
            fs::read_to_string(path).expect("Should have been able to read the file")
        }

        pub fn load_struct_from_json<T: serde::de::DeserializeOwned>(
            file_name: &str,
        ) -> Result<T, Box<dyn Error>> {
            let data = read_file(&helper_module::str!(file_name));

            let obj: Result<T, serde_json::Error> = serde_json::from_str(&data);
            match obj {
                Ok(item) => Ok(item),
                Err(e) => Err(Box::new(e)),
            }
        }

        pub fn serialize_struct_to_json<T: serde::ser::Serialize>(
            obj: &T,
        ) -> Result<String, Box<dyn serde::ser::StdError>> {
            let j = serde_json::to_string(&obj)?;
            Ok(j)
        }

        pub fn write_json_to_file(file_path: &str, json: &str) {
            fs::write(file_path, json).expect("Unable to write file");
        }

        pub fn save_obj<T: serde::ser::Serialize>(file_path: &str, obj: &T) {
            write_json_to_file(file_path, serialize_struct_to_json(&obj).unwrap().as_str());
        }
    }
}

pub mod toolbox {
    use crate::game_features::character::*;
    use crate::game_features::helper_module::save_n_load::load_struct_from_json;

    pub const GOLDSWORD: &str =
        r"C:\Users\julia\OneDrive\Dokumente\GitHub\terminal_rpg\src\assets\items\gold_sword.json";
    pub const BERSERK: &str = r"C:\Users\julia\OneDrive\Dokumente\GitHub\terminal_rpg\src\assets\saved_characters\random_berserker.json";
    pub const IRONSWORD: &str = r"C:\Users\julia\OneDrive\Dokumente\GitHub\terminal_rpg\src\assets\saved_characters\iron_sword.json";

    pub fn load_preset_entity(preset: &str) -> Entity {
        match load_struct_from_json(preset) {
            Ok(item) => item,
            Err(_e) => {
                let params: EntityTuple = (
                    "ERROR ENTITY".to_string(),
                    Races::ErrorRace,
                    Classes::ErrorClass,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    Vec::new(),
                    Vec::new(),
                    Inventory::new(),
                );
                Entity::new(params)
            }
        }
    }

    pub fn load_preset_item(preset: &str) -> Item {
        match load_struct_from_json(preset) {
            Ok(item) => item,
            Err(_e) => Item::new(
                "ERROR ITEM".to_string(),
                ItemTypes::ErrorItemType,
                ItemSubTypes::ErrorItemSubType,
                0,
                0,
                0,
            ),
        }
    }
}
