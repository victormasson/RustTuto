pub fn new_thing(name: &str) {
    println!("");
    println!("> {}", name);
}

pub mod DisplayFerris {
    use ferris_says::say;
    use std::io::{stdout, BufWriter};

    pub fn say_hello() {
        write("Hello fellow Rustaceans!");
    }

    pub fn say_text(text: &str) {
        write(text);
    }

    fn write(text: &str) {
        let stdout = stdout();
        let message = String::from(text);
        let width_message = message.chars().count();
        let width_total = if width_message <= 17 {
            width_message
        } else {
            5 * (width_message / 7)
        };
        let mut writer = BufWriter::new(stdout.lock());
        say(message.as_bytes(), width_total, &mut writer).unwrap();
    }
}

pub mod Collections {
    #[derive(Debug)]
    pub struct Color {
        red: u8,
        green: u8,
        blue: u8,
        alpha: u8,
    }

    impl Color {
        pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
            return Color {
                red: red,
                green: green,
                blue: blue,
                alpha: alpha,
            };
        }

        pub fn format(&self) -> String {
            return format!("{}{}{}", &self.red, &self.green, &self.blue);
        }

        pub fn format_separator(&self, separator: char) -> String {
            return format!(
                "{}{}{}{}{}",
                &self.red, separator, &self.green, separator, &self.blue
            );
        }
    }
}

pub mod SpeedSort {
    use rand::Rng;

    pub fn display(tab: Vec<u8>) {
        for value in tab.iter() {
            print!("{} ", value);
        }
        println!("");
    }

    pub fn speed_sort(tab: &mut Vec<u8>, start_index: usize, end_index: usize) {
        if start_index < end_index {
            let mut rng = rand::thread_rng();
            let mut pivot_index = rng.gen_range(start_index, end_index);
            pivot_index = partitionner(tab, start_index, end_index, pivot_index);
            if pivot_index > start_index + 1 {
                speed_sort(tab, start_index, pivot_index - 1);
            }
            if pivot_index < (end_index - 1) {
                speed_sort(tab, pivot_index + 1, end_index);
            }
        }
    }

    fn partitionner(
        tab: &mut Vec<u8>,
        start_index: usize,
        end_index: usize,
        pivot_index: usize,
    ) -> usize {
        intervertir(tab, pivot_index, end_index);
        let mut final_pivot_index = start_index;
        for i in start_index..end_index {
            if tab[i] <= tab[end_index] {
                intervertir(tab, i, final_pivot_index);
                final_pivot_index = final_pivot_index + 1;
            }
        }
        intervertir(tab, final_pivot_index, end_index);
        return final_pivot_index;
    }

    fn intervertir(tab: &mut Vec<u8>, index1: usize, index2: usize) {
        let temp = tab[index1];
        tab[index1] = tab[index2];
        tab[index2] = temp;
    }
}
