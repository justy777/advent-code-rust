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

fn look_and_say(number: &str) -> String {
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

#[test]
fn test_look_and_say() {
    let result = look_and_say("1");
    assert_eq!(result, "11");

    let result = look_and_say(&result);
    assert_eq!(result, "21");

    let result = look_and_say(&result);
    assert_eq!(result, "1211");

    let result = look_and_say(&result);
    assert_eq!(result, "111221");

    let result = look_and_say(&result);
    assert_eq!(result, "312211");
}

#[test]
fn test_year_2015_day_10() {
    println!("Advent of Code 2015 - Day 10");

    let mut input = String::from("1113122113");
    for _ in 0..40 {
        input = look_and_say(&input);
    }

    let length = input.len();
    println!("The length of the result is {}.", length);
    assert_eq!(length, 360154);

    let mut input = String::from("1113122113");
    for _ in 0..50 {
        input = look_and_say(&input);
    }

    let length = input.len();
    println!("The length of the new result is {}.", length);
    assert_eq!(length, 5103798);
}
