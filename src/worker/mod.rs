use crate::types::{DaysOfMonth, NetcdfFile, WriteCommand};
use std::sync::{mpsc::Sender, Arc};

#[derive(Clone)]
pub struct Worker {
    pub days_of_months: Vec<DaysOfMonth>,
    pub file: Arc<NetcdfFile>,
    pub write_sender: Sender<WriteCommand>,
}

impl Worker {
    pub fn run(&self) {
        let longitudes_len = self.file.longitudes.len();
        let latitudes_len = self.file.latitudes.len();

        for days_of_month in &self.days_of_months {
            for longitude_index in 0..longitudes_len {
                for latitude_index in 0..latitudes_len {
                    let mut sum = 0.0;
                    let mut count = 0;
                    let index_offset = longitude_index + latitude_index * longitudes_len;
                    for day_index in &days_of_month.day_indexes {
                        match self
                            .file
                            .temperatures
                            .get(index_offset + day_index * latitudes_len * longitudes_len)
                        {
                            Some(temp) => {
                                if temp != &self.file.temp_fill_value {
                                    sum +=
                                        temp * self.file.temp_scale_factor + self.file.temp_offset;
                                    count += 1;
                                }
                            }
                            None => eprintln!(
                                "Temp not found in longitude:{} latitude:{} time:{}",
                                longitude_index, latitude_index, day_index,
                            ),
                        }
                    }
                    let average = sum / count as f32;

                    let _ = self.write_sender.send(WriteCommand {
                        sheet_index: days_of_month.index,
                        row: (latitude_index + 1) as u32,
                        column: (longitude_index + 1) as u16,
                        longitude: *self.file.longitudes.get(longitude_index).unwrap() as f64,
                        latitude: *self.file.latitudes.get(latitude_index).unwrap() as f64,
                        temp: average as f64,
                    });
                }
            }
        }
    }
}
