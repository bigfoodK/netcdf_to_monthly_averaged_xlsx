use xlsxwriter::{Workbook, Worksheet};

use crate::types::{DaysOfMonth, WriteCommand};

pub struct SheetWriter<'a> {
    sheets: Vec<Worksheet<'a>>,
}

impl<'a> SheetWriter<'a> {
    pub fn new(workbook: &'a Workbook, divided_months: &Vec<DaysOfMonth>) -> Self {
        let mut sheets = Vec::new();
        for days_of_month in divided_months {
            sheets.push(
                workbook
                    .add_worksheet(Some(
                        format!("{}.{}", days_of_month.year, days_of_month.month).as_str(),
                    ))
                    .unwrap(),
            );
        }
        Self { sheets }
    }

    pub fn write<'b>(&'b mut self, command: &WriteCommand) {
        let sheet = self.sheets.get_mut(command.sheet_index).unwrap();
        let _ = sheet.write_number(0, command.column, command.longitude, None);
        let _ = sheet.write_number(command.row, 0, command.latitude, None);
        if command.temp.is_normal() {
            let _ = sheet.write_number(command.row, command.column, command.temp, None);
        }
    }
}
