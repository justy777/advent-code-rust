use criterion::criterion_main;

mod year_2015;

criterion_main! {
    year_2015::day_01::benches,
    year_2015::day_02::benches,
    year_2015::day_03::benches,
    year_2015::day_04::benches,
    year_2015::day_05::benches,
    year_2015::day_06::benches,
    year_2015::day_07::benches,
    year_2015::day_08::benches,
    year_2015::day_09::benches,
    year_2015::day_10::benches,
    year_2015::day_11::benches,
    year_2015::day_12::benches,
    year_2015::day_13::benches,
    year_2015::day_14::benches,
}
