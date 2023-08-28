use std::vec;

fn main() {
    let years_to_check = vec![2000, 2004, 2100, 2400];

    for year in years_to_check {
        if is_leap_year(year) {
            println!("{:04}年は閏年", year);
        } else {
            println!("{:04}年は閏年ではない", year);
        }
    }
}

fn is_leap_year(year: i32) -> bool {
    match (year % 4, year % 100, year % 400) {
        (_, _, 0) => true,  // 400で割り切れる場合は閏年
        (_, 0, _) => false, // 上の条件を満たさず,100で割り切れる場合は閏年ではない
        (0, _, _) => true,  // 上の条件を満たさず,4で割り切れる場合は閏年
        _ => false,         // それ以外は閏年ではない
    }
}
