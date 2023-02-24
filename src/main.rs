use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!{"{}", "Usage: \nCommand: timecalc 8:00\nOutput: 8.00"};
        println!{"{}", "Command: ./timecalc(.exe) 8:00 ,\nOutput: 8,00\n"};
        println!{"{}", "Command: ./timecalc(.exe) 9.00\nOutput: 9.00"};
        println!{"{}", "Command: ./timecalc(.exe) 9.00 ,\nOutput: 9,00\n"};
        println!{"{}", "Command: ./timecalc(.exe) 10,00\nOutput: 10.00"};
        println!{"{}", "Command: ./timecalc(.exe) 10,00 ,\nOutput: 10,00\n"};
        return;
    }

    let timeline = &args[1];
    let mut decimal_separator = "";

    if args.len() >= 3 {
        decimal_separator = &args[2];
    }

    let sans: String;
    if timeline.find(":") != Option::None {
        sans = timeline.replace(":", ".");
    } else if timeline.find(",") != Option::None {
        sans = timeline.replace(",", ".");
    } else {
        sans = String::from(timeline);
    }


    let time_float = sans.parse::<f32>().unwrap();

    let hours_int : i32  = time_float as u8 as i32;

    let hours = hours_int as f32;
    let mut minutes = (time_float - hours) * 100_f32;

    minutes += hours*60_f32;

    let time_decimal = minutes / 60_f32;
    let time_string: String = format!("{:.2}", time_decimal);

    if decimal_separator == "" {
        println!("{}", time_string);

    } else {
        let new_time_string = time_string.replace(".", decimal_separator);
        println!("{}", new_time_string);
    }



}
