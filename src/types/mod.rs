use chrono::{Date, Utc};

#[derive(Debug, Clone)]
pub struct DaysOfMonth {
    pub index: usize,
    pub year: i16,
    pub month: i16,
    pub day_indexes: Vec<usize>,
}

#[derive(Debug)]
pub struct WriteCommand {
    pub sheet_index: usize,
    pub row: u32,
    pub column: u16,
    pub longitude: f64,
    pub latitude: f64,
    pub temp: f64,
}

pub struct NetcdfFile {
    pub temp_scale_factor: f32,
    pub temp_offset: f32,
    pub temp_fill_value: f32,
    pub date_offset: Date<Utc>,
    pub temperatures: Vec<f32>,
    pub times: Vec<i32>,
    pub longitudes: Vec<f32>,
    pub latitudes: Vec<f32>,
}
