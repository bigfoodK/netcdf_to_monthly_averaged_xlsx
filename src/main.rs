mod netcdf_util;
mod sheet_writer;
mod types;
mod worker;
use chrono::{Duration, Utc};
use clap::{App, Arg};
use netcdf_util::*;
use std::{
    env,
    sync::{mpsc::channel, Arc},
    thread::{spawn, JoinHandle},
};
use xlsxwriter::Workbook;

use crate::{sheet_writer::SheetWriter, types::WriteCommand, worker::Worker};

fn main() {
    let worker_count = num_cpus::get() as u8;
    println!("{} workers", worker_count);

    let matches = App::new("average month temp")
        .arg(
            Arg::new("file_path")
                .required(true)
                .takes_value(true)
                .index(1),
        )
        .get_matches();
    let file_path = matches.value_of("file_path").unwrap();

    let mut absolute_file_path = env::current_dir().unwrap();
    absolute_file_path.push(file_path);
    println!("{:?}", absolute_file_path);

    let file = Arc::new(load_netcdf_file(&absolute_file_path));

    let divided_months = divide_to_month(file.date_offset, &file.times);
    let total_data_count = file.latitudes.len() * file.longitudes.len() * &divided_months.len();
    let workbook = Workbook::new(absolute_file_path.with_extension("xlsx").to_str().unwrap());
    let mut sheet_writer = SheetWriter::new(&workbook, &divided_months);

    let splitted_days_of_months = split_to_worker_count(divided_months, worker_count);
    let (tx, rx) = channel::<WriteCommand>();

    let mut workers: Vec<JoinHandle<()>> = Vec::new();
    for id in 0..worker_count {
        let worker = Worker {
            days_of_months: splitted_days_of_months.get(id as usize).unwrap().clone(),
            file: file.clone(),
            write_sender: tx.clone(),
        };
        workers.push(spawn(move || worker.run()))
    }

    let print_interval = Duration::seconds(1);
    let mut written_data_count = 0;
    let mut last_printed_at = Utc::now();
    let started_at = Utc::now();
    while let Ok(command) = rx.recv() {
        written_data_count += 1;
        let now = Utc::now();
        if now - last_printed_at > print_interval {
            let progress = written_data_count as f32 / total_data_count as f32;
            let elapsed_seconds = (now - started_at).num_seconds();

            last_printed_at = now;
            println!(
                "{:02.2}% {}",
                progress * 100.0,
                get_time_string_from_seconds(elapsed_seconds),
            );
        }

        // if written_rows as f32 / total_rows as f32 * 100.0 > 0.5 {
        //     let _ = workbook.close();
        //     panic!("end for test");
        // }

        sheet_writer.write(&command);

        if written_data_count == total_data_count {
            println!("All done.");
            break;
        }
    }

    for worker in workers {
        let _ = worker.join();
    }

    println!("Saving...");
    match workbook.close() {
        Ok(_) => println!("Saved"),
        Err(error) => eprintln!("Save failed\n{:?}", error),
    };
}

fn get_time_string_from_seconds(seconds: i64) -> String {
    let hour = seconds / 3600;
    let min = (seconds / 60) % 60;
    let sec = seconds % 60;
    if hour != 0 {
        return format!("{:02}:{:02}:{:02}", hour, min, sec);
    }
    return format!("{:02}:{:02}", min, sec);
}
