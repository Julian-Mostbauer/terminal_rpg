/* Refrences, Todos, Infos
    Todo:
    -   Make the game lol

    Refrences:
    -   https://donjon.bin.sh/fantasy/name/#type=set

*/

/* Macros --------------------------------------------------------------------------------------*/
macro_rules! str {
    ($a: expr) => {
        $a.to_string()
    };
}

// -----------------------------------------------------------------------------------------
// CHARACTER CREATION MODULE
// -----------------------------------------------------------------------------------------
pub mod character_creation {
    use super::dices::Dice;
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };

    /* Races --------------------------------------------------------------------------------------*/
    #[derive(Debug)]
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
    #[derive(Debug)]
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
    #[derive(Debug)]
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
    /* Entety struct --------------------------------------------------------------------------------------*/
    #[derive(Debug)]
    pub struct Entity {
        name: String,

        race: Races,
        class: Classes,
        lvl: u32,

        strength: u32,
        dexterity: u32,
        constitution: u32,
        intelligence: u32,
        wisdom: u32,
        charisma: u32,

        abilities: Vec<Abilities>,
    }

    pub struct EntityMaker {}
    impl EntityMaker {
        pub fn random_name() -> String {
            let possible_names: Vec<String> = vec![str!("Alice"), str!("Bob")];
            possible_names[Dice::dn(possible_names.len() as u32) as usize].clone()
        }

        pub fn make_rand_barbarian() -> Entity {
            let barb_abilities: Vec<Abilities> = Vec::new();
            Entity {
                name: Self::random_name(),
                class: Classes::Barbarian,
                race: rand::random(),
                lvl: Dice::d20(),

                strength: Dice::d20(),
                dexterity: Dice::d20(),
                constitution: Dice::d20(),
                intelligence: Dice::d20(),
                wisdom: Dice::d20(),
                charisma: Dice::d20(),

                abilities: barb_abilities,
            }
        }
    }
}

// -----------------------------------------------------------------------------------------
// DICE MODULE
// -----------------------------------------------------------------------------------------
pub mod dices {
    use rand::Rng;

    /* Entety struct --------------------------------------------------------------------------------------*/
    pub struct Dice {}
    impl Dice {
        pub fn d4() -> u32 {
            Self::dn(4)
        }
        pub fn d6() -> u32 {
            Self::dn(6)
        }
        pub fn d8() -> u32 {
            Self::dn(8)
        }
        pub fn d12() -> u32 {
            Self::dn(12)
        }
        pub fn d20() -> u32 {
            Self::dn(20)
        }
        pub fn d100() -> u32 {
            Self::dn(100)
        }
        pub fn dn(n: u32) -> u32 {
            rand::thread_rng().gen_range(1..=n)
        }
    }
}