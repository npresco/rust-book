use std::io;

fn main() {
    // Ask temp number and unit
    let mut unit = String::new();
    let mut correct_unit = true;

    while correct_unit {
        println!("Enter temperature unit F or C:");

        io::stdin().read_line(&mut unit)
            .expect("Faled to read line");

        unit = unit.trim().to_string();

        if unit == "F" || unit == "C" {
            correct_unit = false;
        } else {
            println!("Invalid unit");
            unit = String::new();
        }
    }

    let mut temp = String::new();
    let mut correct_temp_type = true;

    while correct_temp_type {
        println!("Enter temperature:");

        io::stdin().read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => {
                correct_temp_type = false;
                num
            }
            Err(_) => {
                temp = String::new();
                continue;
            }
        };

        if unit == "F" {
            let temp: f64 = (temp - 32.0) / 1.8;
            let new_sign = "C";
            println!("The converted temp is {} {}", temp, new_sign);
        } else {
            let temp: f64 = temp * 1.8 + 32.0;
            let new_sign = "F";
            println!("The converted temp is {} {}", temp, new_sign);
        }
    }
}
