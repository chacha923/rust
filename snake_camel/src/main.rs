use std::env;
use std::string::String;
fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); // 去掉第一个参数
    for argument in args {
        snake2camel(argument.as_str())
    }
}

fn snake2camel(snake: &str) {
    let split = snake.split("_");
    let vec = split.collect::<Vec<&str>>();
    let mut is_first_seg = true;
    let mut result = String::new();

    for s in vec {
        if is_first_seg {
            result.push_str(s.to_ascii_lowercase().as_str());
            is_first_seg = false;
        } else {
            result.push_str(first_char_up(s).as_str());
        }
    }

    println!("{}", result)
}

fn first_char_up(s: &str) -> String {
    let mut first = true;
    let mut result = String::new();
    for c in s.chars() {
        if first {
            first = false;
            result.push(c.to_ascii_uppercase());
        } else {
            result.push(c.to_ascii_lowercase());
        }
    }
    return result;
}
