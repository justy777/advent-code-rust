use std::{cmp, fs};
use std::str::FromStr;
use std::num::ParseIntError;
use std::convert::TryFrom;

struct Present {
    length: i32,
    width: i32,
    height: i32,
}

impl Present {

    fn calculate_wrapping_paper_needed(&self) -> i32 {
        let area_1 = self.length * self.width;
        let area_2 = self.width * self.height;
        let area_3 = self.height * self.length;
        let min_area = cmp::min(area_1, cmp::min(area_2, area_3));
        (2 * (area_1 + area_2 + area_3)) + min_area
    }


    fn calculate_ribbon_needed(&self) -> i32 {
        let needed_for_bow = self.length * self.width * self.height;
        let max = cmp::max(self.length, cmp::max(self.width, self.height));
        let smallest_perimeter = 2 * (self.length + self.width + self.height - max);
        smallest_perimeter + needed_for_bow
    }
}

impl FromStr for Present {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dimensions: Vec<&str> = s.split('x').collect();
        let length = dimensions[0].parse::<i32>()?;
        let width = dimensions[1].parse::<i32>()?;
        let height = dimensions[2].parse::<i32>()?;

        Ok(Present {length, width, height})
    }
}

impl TryFrom<&str> for Present {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let dimensions: Vec<&str> = value.split('x').collect();
        let length = dimensions[0].parse::<i32>()?;
        let width = dimensions[1].parse::<i32>()?;
        let height = dimensions[2].parse::<i32>()?;

        Ok(Present {length, width, height})
    }
}


#[test]
fn test_2015_day_2() {
    println!("Advent of Code 2015 - Day 2");
    let contents = fs::read_to_string("input/2015/day-2.txt")
        .expect("Failed to read file to String");

    let presents: Vec<Present> = contents.lines()
        .map(|line| Present::from_str(line).unwrap())
        .collect();

    let wrapping_paper_needed: i32 = presents.iter()
        .map(|present| present.calculate_wrapping_paper_needed())
        .sum();

    println!("The elves need {} feet of wrapping paper.", wrapping_paper_needed);
    assert_eq!(wrapping_paper_needed, 1586300);

    let ribbon_needed: i32 = presents.iter()
        .map(|present| present.calculate_ribbon_needed())
        .sum();

    println!("The elves need {} feet of ribbon.", ribbon_needed);
    assert_eq!(ribbon_needed, 3737498);
}