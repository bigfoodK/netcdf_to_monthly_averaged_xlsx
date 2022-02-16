use netcdf::File;

pub fn get_variable<'a>(
    file: &'a File,
    default_variable_name: &str,
    variable_names: &mut Vec<String>,
) -> netcdf::Variable<'a> {
    let variable_name = match variable_names
        .iter()
        .position(|name| name == &default_variable_name.to_string())
    {
        Some(index) => {
            variable_names.remove(index);
            default_variable_name.to_string()
        }
        None => {
            println!(
                "Default variable name '{}' not found",
                default_variable_name
            );
            let variable_name = prompt_select_variable_name(variable_names);
            variable_names.remove(
                variable_names
                    .iter()
                    .position(|name| name == &variable_name)
                    .unwrap(),
            );
            variable_name
        }
    };

    file.variable(&variable_name)
        .expect(format!("Could not find variable '{}'", variable_name).as_str())
}

fn prompt_select_variable_name(variable_names: &Vec<String>) -> String {
    loop {
        let mut line = String::new();
        println!("Select variable name");
        for (index, name) in variable_names.iter().enumerate() {
            println!("  {}. {}", index + 1, name);
        }
        std::io::stdin().read_line(&mut line).unwrap();
        match line.trim().parse::<i32>() {
            Ok(index) => match variable_names.get((index - 1) as usize) {
                Some(name) => {
                    println!("'{}' selected", name);
                    return name.clone();
                }
                None => println!("Name not found\nTry again\n"),
            },
            Err(error) => {
                println!("{:?}\nTry again\n", error);
            }
        }
    }
}
