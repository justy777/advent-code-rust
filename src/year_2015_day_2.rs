use std::num::ParseIntError;
use std::str::FromStr;
use std::{cmp, fs};

pub struct Present {
    length: i32,
    width: i32,
    height: i32,
}

impl Present {
    pub fn wrapping_paper_needed(&self) -> i32 {
        let area_1 = self.length * self.width;
        let area_2 = self.width * self.height;
        let area_3 = self.height * self.length;

        let min_area = cmp::min(area_1, cmp::min(area_2, area_3));

        (2 * (area_1 + area_2 + area_3)) + min_area
    }

    pub fn ribbon_needed(&self) -> i32 {
        let needed_for_bow = self.length * self.width * self.height;
        let max = cmp::max(self.length, cmp::max(self.width, self.height));
        let smallest_perimeter = 2 * (self.length + self.width + self.height - max);

        smallest_perimeter + needed_for_bow
    }
}

impl FromStr for Present {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Present, ParseIntError> {
        let dimensions: Vec<&str> = s.split('x').collect();
        let length = dimensions[0].parse::<i32>()?;
        let width = dimensions[1].parse::<i32>()?;
        let height = dimensions[2].parse::<i32>()?;

        Ok(Present {
            length,
            width,
            height,
        })
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
fn test_2015_day_2() {
    println!("Advent of Code 2015 - Day 2");
    let contents =
        fs::read_to_string("input/2015/day-2.txt").expect("Failed to read file to string.");

    let presents: Vec<Present> = contents
        .lines()
        .map(|line| Present::from_str(line).unwrap())
        .collect();

    let wrapping_paper_needed: i32 = presents
        .iter()
        .map(|present| present.wrapping_paper_needed())
        .sum();

    println!(
        "The elves need {} square feet of wrapping paper.",
        wrapping_paper_needed
    );
    assert_eq!(wrapping_paper_needed, 1586300);

    let ribbon_needed: i32 = presents.iter().map(|present| present.ribbon_needed()).sum();

    println!("The elves need {} feet of ribbon.", ribbon_needed);
    assert_eq!(ribbon_needed, 3737498);
}
