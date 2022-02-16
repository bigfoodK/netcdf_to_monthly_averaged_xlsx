use super::{get_variable_attribute_as_f32, get_variable_unit_as_date};
use crate::{netcdf_util::get_variable, types::NetcdfFile};
use std::path::PathBuf;

const DEFAULT_TIME_NAME: &str = "time";
const DEFAULT_LONGITUDE_NAME: &str = "longitude";
const DEFAULT_LATITUDE_NAME: &str = "latitude";

pub fn load_netcdf_file(file_path: &PathBuf) -> NetcdfFile {
    let file = netcdf::open(file_path).unwrap();
    let mut variable_names: Vec<String> =
        file.variables().map(|variable| variable.name()).collect();
    println!("variables {:?}", variable_names);

    let times = get_variable(&file, DEFAULT_TIME_NAME, &mut variable_names);
    let longitudes = get_variable(&file, DEFAULT_LONGITUDE_NAME, &mut variable_names)
        .values(None, None)
        .unwrap()
        .into_raw_vec();
    let latitudes = get_variable(&file, DEFAULT_LATITUDE_NAME, &mut variable_names)
        .values(None, None)
        .unwrap()
        .into_raw_vec();

    let temperatures = match variable_names.len() {
        0 => {
            panic!("No remained variable found.");
        }
        1 => get_variable(
            &file,
            variable_names.first().unwrap().clone().as_str(),
            &mut variable_names,
        ),
        _ => get_variable(&file, "", &mut variable_names),
    };
    let temperature_dimensions: Vec<(String, usize)> = temperatures
        .dimensions()
        .into_iter()
        .map(|dimension| (dimension.name(), dimension.len()))
        .collect();
    println!("{:?}", temperature_dimensions);

    let scale_factor = get_variable_attribute_as_f32(&temperatures, "scale_factor", Some(1.0));
    let add_offset = get_variable_attribute_as_f32(&temperatures, "add_offset", Some(0.0));
    let fill_value = get_variable_attribute_as_f32(&temperatures, "_FillValue", None);
    let date_offset = get_variable_unit_as_date(&times);

    let temperatures = temperatures.values(None, None).unwrap().into_raw_vec();
    let times = times.values(None, None).unwrap().into_raw_vec();

    NetcdfFile {
        temp_scale_factor: scale_factor,
        temp_offset: add_offset,
        temp_fill_value: fill_value,
        date_offset,
        temperatures,
        times,
        longitudes,
        latitudes,
    }
}
