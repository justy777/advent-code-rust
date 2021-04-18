use regex::Regex;
use std::str::FromStr;

struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl FromStr for Ingredient {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^(?P<name>\w+): capacity (?P<capacity>-?\d+), durability (?P<durability>-?\d+), flavor (?P<flavor>-?\d+), texture (?P<texture>-?\d+), calories (?P<calories>-?\d+)$").unwrap();
        }
        match REGEX.captures(s) {
            Some(caps) => {
                let name = caps.name("name").unwrap().as_str().to_string();

                let parse_int = |key| caps.name(key).unwrap().as_str().parse::<i32>().unwrap();

                let capacity = parse_int("capacity");
                let durability = parse_int("durability");
                let flavor = parse_int("flavor");
                let texture = parse_int("texture");
                let calories = parse_int("calories");

                Ok(Ingredient {
                    name,
                    capacity,
                    durability,
                    flavor,
                    texture,
                    calories,
                })
            }
            None => Err(()),
        }
    }
}
