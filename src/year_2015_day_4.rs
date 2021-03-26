use std::fs;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn find_md5_hash_leading_zeroes(key: &[u8], leading_zeroes: u64) -> Option<u64> {
    let mut hasher = Md5::new();

    let leading_bytes = if leading_zeroes % 2 == 0 {leading_zeroes / 2} else {leading_zeroes / 2 + 1};

    let mut result = None;
    for i in 0..u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);

        let mut counter = leading_zeroes;
        let mut first_bytes = 0;
        for byte in output.iter().take(leading_bytes as usize) {
            if counter >= 2 {
                first_bytes += *byte as u64;
                counter -= 2;
            } else {
                first_bytes += (*byte >> 4) as u64;
                counter -= 1;
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
fn test_2015_day_4() {
    println!("Advent of Code 2015 - Day 4");
    let contents = fs::read_to_string("input/2015/day-4.txt")
        .expect("Failed to read file to string.");

    let second_half_of_key = find_md5_hash_leading_zeroes(contents.as_bytes(), 5).unwrap();
    println!("The secret key is {}, and the answer is {} for an MD5 hash with five leading zeroes.", contents, second_half_of_key);
    assert_eq!(second_half_of_key, 346386);

    let second_half_of_key = find_md5_hash_leading_zeroes(contents.as_bytes(), 6).unwrap();
    println!("The secret key is {}, and the answer is {} for an MD5 hash with six leading zeroes.", contents, second_half_of_key);
    assert_eq!(second_half_of_key, 9958218);
}