use adventlib;

pub fn rocket_equation(file: &String) {
    println!("The Tyranny of the Rocket Equation\n");

    let module_masses = adventlib::read_file_as_i64(file);

    let mut required_fuel: i64 = 0;

    for mass in &module_masses {
        required_fuel += (mass / 3) - 2;
    }

    println!("Naive Total Fuel Requirements: {}", required_fuel);

    let mut grand_total: i64 = 0;

    for mass in &module_masses {
        let mut module_total_fuel = (mass / 3) - 2;
        let mut fuel_mass = module_total_fuel;
        while fuel_mass > 0 {
            let additional_fuel = (fuel_mass / 3) - 2;
            if additional_fuel > 0 {
                module_total_fuel += additional_fuel;
            }
            fuel_mass = additional_fuel;
        }
        grand_total += module_total_fuel;
    }

    println!("Real Total Fuel Requirements: {}", grand_total);

}