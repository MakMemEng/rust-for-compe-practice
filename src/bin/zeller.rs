//! y年 m月 d日
//! Y = y mod 100
//! C = y / 100
//! Γ = 5C + C/4    :Gregorian(1582 ≦ y)
//!     6C + 5      :Julian(4 ≦ y ≦ 1582)
//! D = [{d + [26(m + 1)/10] + Y + Y/4 + Γ + 5} mod 7] + 1
//! :Parameter
//!     y: year
//!     century
//!     gamma
//!     d: date
//! :Return
//!     D: 曜日 月曜:1...日曜:7
// mod 7: 曜日の数, 循環する仕組み
// [26(m+1)/10]: 月の計算  1月を前年の13月, 2月を前年の14月とする(ユリウス暦,グレゴリオ暦の調整の為)
// Y+[Y/4]+Γ: 年の計算
// +5, +1 ISO 8601に準拠するための調整

enum DayOfWeek {
    Sunday,    // 日曜日
    Monday,    // 月曜日
    Tuesday,   // 火曜日
    Wednesday, // 水曜日
    Thursday,  // 木曜日
    Friday,    // 金曜日
    Saturday,  // 土曜日
}

impl DayOfWeek {
    fn from_iso_number(iso_number: i32) -> DayOfWeek {
        match iso_number {
            1 => DayOfWeek::Monday,
            2 => DayOfWeek::Tuesday,
            3 => DayOfWeek::Wednesday,
            4 => DayOfWeek::Thursday,
            5 => DayOfWeek::Friday,
            6 => DayOfWeek::Saturday,
            7 => DayOfWeek::Sunday,
            _ => unreachable!(),
        }
    }

    fn to_ja(&self) -> char {
        match self {
            DayOfWeek::Monday => '月',
            DayOfWeek::Tuesday => '火',
            DayOfWeek::Wednesday => '水',
            DayOfWeek::Thursday => '木',
            DayOfWeek::Friday => '金',
            DayOfWeek::Saturday => '土',
            DayOfWeek::Sunday => '日',
        }
    }
}
fn main() {
    let year = 2023;
    let month = 8;
    let day = 4;

    let zeller_iso = zeller(year, month, day);
    let day_of_week = DayOfWeek::from_iso_number(zeller_iso);
    let day_of_week_ja = day_of_week.to_ja();
    println!("{:04}-{:02}-{:02} ({})", year, month, day, day_of_week_ja);
}

fn zeller(y: i32, m: i32, d: i32) -> i32 {
    let century = y / 100;
    let gamma = if (4..1582).contains(&y) {
        6 * century + 5 // ユリウス暦
    } else {
        5 * century + century / 4 // グレゴリオ暦
    };
    // 1, 2月は前年の13, 14月として扱う
    let (year, month) = if m <= 2 { (y - 1, m + 12) } else { (y, m) };
    let year_12 = year % 100; // 西暦の下二桁

    // ツェラーの公式で計算(ISO 8601 形式; 1〜7月〜日)
    (d + (26 * (month + 1)) / 10 + year_12 + (year_12 / 4) + gamma + 5) % 7 + 1
}
