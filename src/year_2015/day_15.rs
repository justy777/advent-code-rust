use regex::Regex;
use std::str::FromStr;

use crate::util::CapturesWrapper;

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
                let caps = CapturesWrapper::new(caps);
                let name = caps.as_string("name");
                let capacity = caps.parse("capacity");
                let durability = caps.parse("durability");
                let flavor = caps.parse("flavor");
                let texture = caps.parse("texture");
                let calories = caps.parse("calories");

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
