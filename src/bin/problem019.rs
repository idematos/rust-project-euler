// Problem #19: Counting Sundays
// https://projecteuler.net/problem=19

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn main() {
    let month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut day_of_week = 1; 
    let mut sunday_count = 0;

    for year in 1901..=2000 {
        for (month, &days) in month_days.iter().enumerate() {
            if day_of_week == 0 {
                sunday_count += 1;
            }

            let mut days_in_month = days;
            if month == 1 && is_leap_year(year) {
                days_in_month += 1; 
            }

            day_of_week = (day_of_week + days_in_month) % 7;
        }
    }

    println!("The number of sundays on the first of the month is {}", sunday_count);
}
