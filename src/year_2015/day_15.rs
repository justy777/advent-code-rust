use std::str::FromStr;

use regex::Regex;

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
                let capacity = caps
                    .name("capacity")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                let durability = caps
                    .name("durability")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                let flavor = caps
                    .name("flavor")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                let texture = caps
                    .name("texture")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                let calories = caps
                    .name("calories")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();

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
