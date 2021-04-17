fn get_count(s: &[u8], c: &u8) -> usize {
    let mut count = 0;
    for n in s {
        if n == c {
            count += 1
        } else {
            break;
        }
    }
    count
}

pub fn look_and_say(number: &str) -> String {
    let mut result: Vec<u8> = Vec::new();

    let bytes = number.as_bytes();
    let mut numbers = number.as_bytes().iter().peekable();
    let mut i = 0;
    while numbers.peek() != None {
        let c = numbers.next().unwrap();
        let number = get_count(&bytes[i..], c);
        i += number;
        number
            .to_string()
            .as_bytes()
            .iter()
            .for_each(|b| result.push(*b));
        result.push(*c);

        for _ in 0..(number - 1) {
            numbers.next();
        }
    }
    String::from_utf8(result).unwrap()
}
