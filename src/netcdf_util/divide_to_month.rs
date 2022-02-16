use crate::types::DaysOfMonth;
use chrono::{Date, Datelike, Duration, Utc};

pub fn divide_to_month(offset: Date<Utc>, days: &Vec<i32>) -> Vec<DaysOfMonth> {
    if days.is_empty() {
        panic!("No days in time values");
    }

    let first_date = offset + Duration::days(*days.get(0).unwrap() as i64);
    let mut index: usize = 0;
    let mut year: i32 = first_date.year();
    let mut month: u32 = first_date.month();
    let mut day_indexes_of_month: Vec<usize> = Vec::new();
    let mut months: Vec<DaysOfMonth> = Vec::new();

    for (day_index, day) in days.into_iter().enumerate() {
        let date = offset + Duration::days(*day as i64);
        if (year != date.year()) || (month != date.month()) {
            months.push(DaysOfMonth {
                index,
                year: year as i16,
                month: month as i16,
                day_indexes: day_indexes_of_month.clone(),
            });
            index += 1;
            year = date.year();
            month = date.month();
            day_indexes_of_month.clear();
        }

        day_indexes_of_month.push(day_index as usize);
    }

    months.push(DaysOfMonth {
        index,
        year: year as i16,
        month: month as i16,
        day_indexes: day_indexes_of_month.clone(),
    });

    months
}
