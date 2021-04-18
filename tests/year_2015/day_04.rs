use advent_of_code::year_2015::day_04::find_number;

#[test]
fn test_find_number_with_five_leading_zeroes_input() {
    let number = find_number(b"iwrupvqb", 5);
    assert_eq!(number, Some(346386));
}

#[test]
fn test_find_number_with_six_leading_zeroes_input() {
    let number = find_number(b"iwrupvqb", 6);
    assert_eq!(number, Some(9958218));
}
