/*!
--- Day 2: I Was Told There Would Be No Math ---
The elves are running low on wrapping paper, and so they need to submit an order for more.
They have a list of the dimensions of each present, and only want to order exactly as much as they need.
*/

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::util::CapturesWrapper;

/// Represents the dimensions of a present in feet.
#[derive(Debug)]
pub struct Present {
    pub length: u32,
    pub width: u32,
    pub height: u32,
}

impl Present {
    /// Calculates the wrapping paper needed to wrap the present in square feet.
    ///
    /// The wrapping paper needed is the surface area of the box plus the area of the smallest side.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use advent_of_code::year_2015::day_02::Present;
    ///
    /// let present = Present::from_str("2x3x4").unwrap();
    /// let paper = present.wrapping_paper_needed();
    /// assert_eq!(paper, 58);
    ///
    /// let present = Present::from_str("1x1x10").unwrap();
    /// let ribbon = present.wrapping_paper_needed();
    /// assert_eq!(ribbon, 43);
    /// ```
    #[must_use]
    pub fn wrapping_paper_needed(&self) -> u32 {
        let area_1 = self.length * self.width;
        let area_2 = self.width * self.height;
        let area_3 = self.height * self.length;

        let min_area = area_1.min(area_2).min(area_3);

        2 * (area_1 + area_2 + area_3) + min_area
    }

    /// Calculates the ribbon needed to wrap the present in feet.
    /// Ribbon is all the same width, so you only need to worry about the length.
    ///
    /// The ribbon required to wrap a present is the shortest distance around its sides, or the smallest perimeter of any one face.
    /// Each present also requires a bow made out of ribbon as well; the feet of ribbon required for the perfect bow is equal to the cubic feet of volume of the present.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use advent_of_code::year_2015::day_02::Present;
    ///
    /// let present = Present::from_str("2x3x4").unwrap();
    /// let ribbon = present.ribbon_needed();
    /// assert_eq!(ribbon, 34);
    ///
    /// let present = Present::from_str("1x1x10").unwrap();
    /// let ribbon = present.ribbon_needed();
    /// assert_eq!(ribbon, 14);
    /// ```
    #[must_use]
    pub fn ribbon_needed(&self) -> u32 {
        let needed_for_bow = self.length * self.width * self.height;
        let max = self.length.max(self.width).max(self.height);
        let smallest_perimeter = 2 * (self.length + self.width + self.height - max);

        smallest_perimeter + needed_for_bow
    }
}

impl FromStr for Present {
    type Err = ParsePresentError;

    fn from_str(s: &str) -> Result<Present, ParsePresentError> {
        static REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^(?P<length>\d+)x(?P<width>\d+)x(?P<height>\d+)$").unwrap());
        match REGEX.captures(s) {
            Some(caps) => {
                let caps = CapturesWrapper::new(caps);
                let length = caps.parse("length");
                let width = caps.parse("width");
                let height = caps.parse("height");

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

/// Error type used when parsing a present from a `str`.
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
