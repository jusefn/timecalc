use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let timeline = &args[1];



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
    let mut time_string: String = format!("{:.2}", time_decimal);
    time_string = time_string.replace(".", ",");

    println!("{}", time_string);
}
