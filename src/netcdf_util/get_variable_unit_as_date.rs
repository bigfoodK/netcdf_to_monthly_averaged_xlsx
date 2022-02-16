use chrono::{Date, TimeZone, Utc};
use netcdf::Variable;

pub fn get_variable_unit_as_date(variable: &Variable) -> Date<Utc> {
    match variable
        .attribute("units")
        .expect(format!("No attribute units exist in {}", variable.name()).as_str())
        .value()
        .unwrap()
    {
        netcdf::AttrValue::Str(string) => {
            let mut year_month_day = string.split(" ").nth(2).unwrap().split("-");
            Utc.ymd(
                (&year_month_day.nth(0)).unwrap().parse::<i32>().unwrap(),
                (&year_month_day.nth(0)).unwrap().parse::<u32>().unwrap(),
                (&year_month_day.nth(0)).unwrap().parse::<u32>().unwrap(),
            )
        }
        _ => panic!("Could not convert attribute to date"),
    }
}
