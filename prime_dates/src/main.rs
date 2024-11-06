use std::env;
use std::fs::File;
use std::io::{self, Write};

#[derive(PartialEq)]
struct Date(u8,u8,u16);


impl Date{
    fn next(&mut self) {
        if self.day() == self.days_this_month() {
            self.0 = 1;
            self.1 += 1;
            if self.month() == 13 {
                self.1 = 1;
                self.2 += 1;
            }
        }
        else {
            self.0 += 1;
        }
    }
    
    fn day(&self) -> u8 {
        self.0
    } 
    
    fn month(&self) -> u8 {
        self.1
    } 
    
    fn year(&self) -> u16 {
        self.2
    } 
    
    fn to_number(&self) -> u64 {
        let number = self.day() as u64;
        let number = number * 100 + self.month() as u64;
        let number = number * 10000 + self.year() as u64;
        number
    }

    fn days_this_month(&self) -> u8 {
        match self.month() {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            3..=12 => 30,
            2 => {
                if self.is_leap_year() {
                    29
                }
                else {
                    28
                }
            },
            _ => panic!("got bad month!")
        }
    }

    fn is_leap_year(&self) -> bool {
        match self.year() {
            year if year % 400 == 0 => true,
            year if year % 100 == 0 => false,
            year if year % 4 == 0 => true,
            _ => false
        }
    }
}

impl From<String> for Date {
    fn from(date_string: String) -> Self {
        let date_components: Vec<&str> = date_string.split('-').map(|x| x.trim()).collect();
        let day = date_components[0].parse::<u8>().unwrap();
        let month = date_components[1].parse::<u8>().unwrap();
        let year = date_components[2].parse::<u16>().unwrap();
        Date(day, month, year)
    }
}

impl std::cmp::PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.year() < other.year() {
            Some(std::cmp::Ordering::Less)
        }
        else if self.year() > other.year() {
            Some(std::cmp::Ordering::Greater)
        }
        else {
            if self.month() < other.month() {
                Some(std::cmp::Ordering::Less)
            }
            else if self.month() > other.month() {
                Some(std::cmp::Ordering::Greater)
            }
            else {
                if self.day() < other.day() {
                    Some(std::cmp::Ordering::Less)
                }
                else if self.day() > other.day() {
                    Some(std::cmp::Ordering::Greater)
                }
                else {
                    Some(std::cmp::Ordering::Equal)
                }
            }
        }
    }
}

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);

    let mut dates_split = buffer.split(' ');
    
    let output_path = env::var("OUTPUT_PATH");
    let output_path = match output_path {
        Ok(val) => val,
        Err(_) => String::from("output.txt")
    };
    let mut fptr = File::create(output_path).unwrap();


    let date1 = Date::from(dates_split.next().unwrap().to_string());
    let date2 = Date::from(dates_split.next().unwrap().to_string());

    let lucky_dates = lucky_dates_between(date1, date2);
    
    writeln!(&mut fptr, "{}", lucky_dates).ok();
}

fn lucky_dates_between(mut date1: Date,mut date2: Date) -> u32 {
    let mut lucky_dates = 0;
    if date1 > date2 {
        std::mem::swap(&mut date1, &mut date2);
    }
    while date1 <= date2 {
        let date_number = date1.to_number();
        if is_lucky(date_number) {
            lucky_dates += 1;
        }
        date1.next();
    }
    lucky_dates
}

fn is_lucky(number: u64) -> bool {
    number % 4 == 0 || number % 7 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_next () {
        let mut date = Date(30,12,2020);
        date.next();
        assert_eq!(date.day(), 31);
        assert_eq!(date.month(), 12);
        assert_eq!(date.year(), 2020);
        date.next();
        assert_eq!(date.day(), 1);
        assert_eq!(date.month(), 1);
        assert_eq!(date.year(), 2021);
        let mut date = Date(28,2,2020);
        date.next();
        assert_eq!(date.day(), 29);
        assert_eq!(date.month(), 2);
        assert_eq!(date.year(), 2020);
        date.next();
        assert_eq!(date.day(), 1);
        assert_eq!(date.month(), 3);
        assert_eq!(date.year(), 2020);
    }

    #[test]
    fn test_is_leap_year() {
        let mut date = Date(1,1,2020);
        assert_eq!(date.is_leap_year(), true);
        date.2 = 2400;
        assert_eq!(date.is_leap_year(), true);
        date.2 = 1900;
        assert_eq!(date.is_leap_year(), false);
        date.2 = 2100;
        assert_eq!(date.is_leap_year(), false);
    }

    #[test]
    fn test_days_this_month() {
        let mut date = Date(1,1,2020);
        assert_eq!(date.days_this_month(), 31);
        date.1 = 2;
        assert_eq!(date.days_this_month(), 29);
        date.1 = 4;
        assert_eq!(date.days_this_month(), 30);
    }

    #[test]
    fn test_to_number() {
        let date = Date(1,1,2020);
        assert_eq!(date.to_number(), 1012020);
        let date = Date(31,12,9999);
        assert_eq!(date.to_number(), 31129999);

    }

    #[test]
    fn test_date_from_string() {
        let date_string = String::from("1-1-2020");
        let date = Date::from(date_string);
        assert_eq!(date.day(), 1);
        assert_eq!(date.month(), 1);
        assert_eq!(date.year(), 2020);
    }

    #[test]
    fn test_lucky_dates_between_0() {
        let date1 = Date(2,8,2025);
        let date2 = Date(4,9,2025);
        assert_eq!(lucky_dates_between(date1, date2), 5);
    }

    #[test]
    fn test_lucky_dates_between_1() {
        let date1 = Date(13,10,2323);
        let date2 = Date(11,10,9536);
        assert_eq!(lucky_dates_between(date1, date2), 942182);
    }

    #[test]
    fn test_lucky_dates_between_2() {
        let date1 = Date(25,6,2365);
        let date2 = Date(7,9,8847);
        assert_eq!(lucky_dates_between(date1, date2), 846403);
    }

    #[test]
    fn test_lucky_dates_between_3() {
        let date1 = Date(10,12,7733);
        let date2 = Date(22,1,7937);
        assert_eq!(lucky_dates_between(date1, date2), 26597);
    }

    #[test]
    fn test_lucky_dates_between_4() {
        let date1 = Date(18,8,4265);
        let date2 = Date(1,9,9400);
        assert_eq!(lucky_dates_between(date1, date2), 670604); 
    }

    #[test]
    fn test_lucky_dates_between_5() {
        let date1 = Date(27,12,9323);
        let date2 = Date(22,4,9960);
        assert_eq!(lucky_dates_between(date1, date2), 83174);
    }

    #[test]
    fn test_lucky_dates_between_single_lucky_day() {
        let date1 = Date(2,8,2024);
        let date2 = Date(2,8,2024);
        assert_eq!(lucky_dates_between(date1, date2), 1);
    }

}