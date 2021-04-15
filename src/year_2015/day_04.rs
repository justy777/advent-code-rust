/*!
--- Day 4: The Ideal Stocking Stuffer ---

Santa needs help mining some AdventCoins (similar to bitcoins) to use as gifts for all the economically forward-thinking little girls and boys.

To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes.
To mine AdventCoins, you must find Santa the lowest positive number that produces such a hash.
*/

use crypto::digest::Digest;
use crypto::md5::Md5;

/// Finds the lowest number that when added to the provided key has an MD5 hash with the provided number of leading zeroes.
///
/// # Examples
///
/// ```
/// use advent_of_code::year_2015::day_04::find_number;
///
/// let number = find_number(b"abcdef", 5);
/// assert_eq!(number, Some(609043));
/// ```
pub fn find_number(key: &[u8], leading_zeroes: u8) -> Option<u64> {
    if leading_zeroes > 128 {
        return None;
    }

    let mut hasher = Md5::new();

    // Find the number of leading zeroes to check for taking into account an odd number of bytes
    let (is_even, leading_bytes) = if leading_zeroes % 2 == 0 {
        (true, leading_zeroes / 2)
    } else {
        (false, leading_zeroes / 2 + 1)
    };

    let mut result = None;
    for i in 0..u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);

        // Adds together the first n bytes of the MD5 hash
        let mut first_bytes = 0;
        for (i, byte) in output.iter().take(leading_bytes as usize).enumerate() {
            if !is_even && i == (leading_bytes - 1) as usize {
                first_bytes += (*byte >> 4) as u64;
            } else {
                first_bytes += *byte as u64;
            }
        }

        if first_bytes == 0 {
            result = Some(i);
            break;
        }

        hasher.reset();
    }
    result
}

#[test]
fn test_find_number_with_five_leading_zeroes() {
    let number = find_number(b"abcdef", 5).unwrap();
    assert_eq!(number, 609043);

    let number = find_number(b"pqrstuv", 5).unwrap();
    assert_eq!(number, 1048970);
}

#[test]
fn test_find_number_with_five_leading_zeroes_input() {
    let key = b"iwrupvqb";

    let number = find_number(key, 5).unwrap();
    assert_eq!(number, 346386);
}
#[test]
fn test_find_number_with_six_leading_zeroes_input() {
    let key = b"iwrupvqb";

    let number = find_number(key, 6).unwrap();
    assert_eq!(number, 9958218);
}
