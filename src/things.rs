use rand::Rng;
use std::time::SystemTime;

#[path = "tool.rs"]
mod tool;
use tool::Collections::Color;
use tool::DisplayFerris::{say_hello, say_text};
use tool::SpeedSort;

pub fn things() {
    tool::new_thing("Ferry");
    say_hello();
    say_text("Hello it's John !");

    tool::new_thing("String");
    let var_string = String::from("Hello");
    let name = "John";

    let res = format!("{} {} :", var_string, name);
    println!("{}", res);

    tool::new_thing("Rand");
    let new_val: i32 = rand::random();
    let new_val2: f32 = rand::random();
    println!("{} {}", new_val, new_val2);

    tool::new_thing("Tab");
    let tab: [i32; 5] = [50; 5];
    println!("{:?}", tab);

    tool::new_thing("Tuple");
    let tuple: (u8, f32, char, &str) = (255, 123.876, '!', "Hey!");
    println!("{:#?}", tuple);

    tool::new_thing("Color");
    let color1 = Color::new(100, 57, 2, 23);
    println!("{:?}", color1);
    println!("{}", color1.format());
    println!("{}", color1.format_separator(';'));

    tool::new_thing("Vectors");
    let mut rng_display = rand::thread_rng();
    let mut items_vec_display = Vec::new();
    for _ in 0..10 {
        items_vec_display.push(rng_display.gen::<u8>());
    }
    SpeedSort::display(items_vec_display);

    tool::new_thing("Vectors sort");
    let mut rng = rand::thread_rng();
    let mut items_vec = Vec::new();
    for _ in 0..100_000 {
        items_vec.push(rng.gen::<u8>());
    }
    let len_vec = items_vec.len() - 1;
    println!("len: {}", len_vec + 1);
    println!("val ->");
    let mut other_vec = items_vec.to_vec();

    let now_speed_sort = SystemTime::now();
    SpeedSort::speed_sort(&mut items_vec, 0, len_vec);
    match now_speed_sort.elapsed() {
        Ok(elapsed) => {
            println!("SpeedSort in {}ms", elapsed.as_millis());
        }
        _ => {}
    };

    let now_sort = SystemTime::now();
    other_vec.sort();
    match now_sort.elapsed() {
        Ok(elapsed) => {
            println!("Sort in {}ms", elapsed.as_millis());
        }
        _ => {}
    };
}

// -> Speedsort
