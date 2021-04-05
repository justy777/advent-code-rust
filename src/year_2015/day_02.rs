use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::{cmp, fmt, fs};

use rayon::prelude::*;
use regex::Regex;

pub struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    pub fn wrapping_paper_needed(&self) -> u32 {
        let area_1 = self.length * self.width;
        let area_2 = self.width * self.height;
        let area_3 = self.height * self.length;

        let min_area = cmp::min(area_1, cmp::min(area_2, area_3));

        (2 * (area_1 + area_2 + area_3)) + min_area
    }

    pub fn ribbon_needed(&self) -> u32 {
        let needed_for_bow = self.length * self.width * self.height;
        let max = cmp::max(self.length, cmp::max(self.width, self.height));
        let smallest_perimeter = 2 * (self.length + self.width + self.height - max);

        smallest_perimeter + needed_for_bow
    }
}

#[derive(Debug)]
pub struct ParsePresentError {
    pub(super) _priv: (),
}

impl Error for ParsePresentError {}

impl Display for ParsePresentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "provided string was not in format '{length}x{width}x{height}'".fmt(f)
    }
}

impl FromStr for Present {
    type Err = ParsePresentError;

    fn from_str(s: &str) -> Result<Present, ParsePresentError> {
        lazy_static! {
            static ref REGEX: Regex =
                Regex::new(r"^(?P<length>\d+)x(?P<width>\d+)x(?P<height>\d+)$").unwrap();
        }
        match REGEX.captures(s) {
            Some(caps) => {
                let parse = |key| caps.name(key).unwrap().as_str().parse::<u32>().unwrap();
                let length = parse("length");
                let width = parse("width");
                let height = parse("height");

                Ok(Present {
                    length,
                    width,
                    height,
                })
            }
            None => Err(ParsePresentError { _priv: () }),
        }
    }
}

#[test]
fn test_wrapping_paper_needed() {
    let present = Present::from_str("2x3x4").unwrap();
    assert_eq!(present.wrapping_paper_needed(), 58);

    let present = Present::from_str("1x1x10").unwrap();
    assert_eq!(present.wrapping_paper_needed(), 43);
}

#[test]
fn test_ribbon_needed() {
    let present = Present::from_str("2x3x4").unwrap();
    assert_eq!(present.ribbon_needed(), 34);

    let present = Present::from_str("1x1x10").unwrap();
    assert_eq!(present.ribbon_needed(), 14);
}

#[test]
fn test_present_from_string_bad_input() {
    match Present::from_str("hjhjxjhjhxikjk") {
        Ok(_) => panic!(),
        Err(_) => (),
    }

    match Present::from_str("1x1") {
        Ok(_) => panic!(),
        Err(_) => (),
    }

    match Present::from_str("1x1x1x1") {
        Ok(_) => panic!(),
        Err(_) => (),
    }

    match Present::from_str("-1x-1x-1") {
        Ok(_) => panic!(),
        Err(_) => (),
    }
}

#[test]
fn test_present_from_string_zero() {
    let present = Present::from_str("0x0x0").unwrap();
    assert_eq!(present.wrapping_paper_needed(), 0);
    assert_eq!(present.ribbon_needed(), 0);
}

#[test]
fn test_2015_day_2() {
    println!("Advent of Code 2015 - Day 2");
    let contents =
        fs::read_to_string("input/2015/day-2.txt").expect("Failed to read file to string.");

    let presents: Vec<Present> = contents
        .par_lines()
        .map(|line| Present::from_str(line).unwrap())
        .collect();

    let wrapping_paper_needed: u32 = presents
        .par_iter()
        .map(|present| present.wrapping_paper_needed())
        .sum();

    println!(
        "The elves need {} square feet of wrapping paper.",
        wrapping_paper_needed
    );
    assert_eq!(wrapping_paper_needed, 1586300);

    let ribbon_needed: u32 = presents
        .par_iter()
        .map(|present| present.ribbon_needed())
        .sum();

    println!("The elves need {} feet of ribbon.", ribbon_needed);
    assert_eq!(ribbon_needed, 3737498);
}
