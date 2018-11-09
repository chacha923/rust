use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;



    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    for val in v1_iter{
        println!("Got: {}",val);
    }
}

//fn simulated_expensive_calculation(intensity: u32) -> u32{
//    println!("calculating slowly...");
//    thread::sleep(Duration::from_secs(2));
//    intensity
//}

fn generate_workout (intensity: u32, random_number: u32){
//    let expensive_result =
//        simulated_expensive_calculation(intensity);

    let expensive_closure = Cacher::new (|num:u32| -> u32{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });


    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

struct Cacher<T> where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>
}

impl <T> Cacher<T> where T: Fn(u32) -> u32{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match  self.value{
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}

impl Iterator for Counter{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item>{
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}