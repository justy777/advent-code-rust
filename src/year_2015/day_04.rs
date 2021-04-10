use crypto::digest::Digest;
use crypto::md5::Md5;

pub fn find_md5_hash_leading_zeroes(key: &[u8], leading_zeroes: u8) -> Option<u64> {
    if leading_zeroes > 128 {
        return None;
    }

    let mut hasher = Md5::new();

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
fn test_find_md5_hash_with_five_leading_zeroes() {
    let value = find_md5_hash_leading_zeroes("abcdef".as_bytes(), 5).unwrap();
    assert_eq!(value, 609043);

    let value = find_md5_hash_leading_zeroes("pqrstuv".as_bytes(), 5).unwrap();
    assert_eq!(value, 1048970);
}

#[test]
fn test_find_md5_with_five_leading_zeroes_input() {
    let key = "iwrupvqb";

    let second_half_of_key = find_md5_hash_leading_zeroes(key.as_bytes(), 5).unwrap();
    assert_eq!(second_half_of_key, 346386);
}
#[test]
fn test_find_md5_with_six_leading_zeroes_input() {
    let key = "iwrupvqb";

    let second_half_of_key = find_md5_hash_leading_zeroes(key.as_bytes(), 6).unwrap();
    assert_eq!(second_half_of_key, 9958218);
}
