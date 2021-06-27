use advent_of_code::year_2015::day_10::look_and_say;

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
fn test_look_and_say_input_40_times() {
    let mut input = String::from("1113122113");
    for _ in 0..40 {
        input = look_and_say(&input);
    }

    let length = input.len();
    assert_eq!(length, 360154);
}

#[test]
fn test_look_and_say_input_50_times() {
    let mut input = String::from("1113122113");
    for _ in 0..50 {
        input = look_and_say(&input);
    }

    let length = input.len();
    assert_eq!(length, 5103798);
}

#[test]
fn test_look_and_say_letters_and_symbols() {
    let value = look_and_say(&String::from("%%^&hfh"));
    assert_eq!(value, "2%1^1&1h1f1h");
}
