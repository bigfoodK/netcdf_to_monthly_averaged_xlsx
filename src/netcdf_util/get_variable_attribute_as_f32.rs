use netcdf::Variable;

pub fn get_variable_attribute_as_f32(
    variable: &Variable,
    attribute_name: &str,
    default_value: Option<f32>,
) -> f32 {
    if let Some(variable) = variable.attribute(attribute_name) {
        return match variable.value().unwrap() {
            netcdf::AttrValue::Short(value) => value as f32,
            netcdf::AttrValue::Uchar(value) => value as f32,
            netcdf::AttrValue::Schar(value) => value as f32,
            netcdf::AttrValue::Ushort(value) => value as f32,
            netcdf::AttrValue::Uint(value) => value as f32,
            netcdf::AttrValue::Int(value) => value as f32,
            netcdf::AttrValue::Ulonglong(value) => value as f32,
            netcdf::AttrValue::Longlong(value) => value as f32,
            netcdf::AttrValue::Float(value) => value as f32,
            netcdf::AttrValue::Double(value) => value as f32,
            _ => panic!("Could not convert attribute to f64"),
        };
    }
    match default_value {
        Some(default_value) => default_value,
        None => {
            panic!(
                "No attribute {} exist in {}. And no default value served",
                attribute_name,
                variable.name()
            );
        }
    }
}
