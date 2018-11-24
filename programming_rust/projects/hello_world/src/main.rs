
fn main() {
	println!("Hello world!");
	print_number(5);
	print_sum(5,6);

	let mut y= 5;
	y = add_one(y);
	println!("{}",y);

	let _f: fn(i32) -> i32;

	let _f = add_one(6);
	let z = _f;
	println!("{}",z);

	str_len();
	for_while();
	for_in();
	for_rev();
	owner();

	let mut v: Vec<i32> = Vec::new();
	v.push(5);

	let mut s = String::new();
	let data = "initial contents";
	let s = data.to_string();

	let mut s = String::from("foo");
	s.push_str("bar");


}

fn print_number(x: i32) {
	println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
	println!("sum is: {}", x + y);
}

fn add_one(x: i32) -> i32 {
	x + 1
}

fn str_len(){
	let space = "space";
	println!("space len is {}",space.len())
}

fn for_while(){
	let a = [10, 20, 30, 40, 50];
	let mut index = 0;

	while index < 5 {
		println!("the value is: {}", a[index]);

		index = index + 1;
	}
}

fn for_in(){
	let a = [10, 20, 30, 40, 50];
	for element in a.iter(){
		println!("the value is: {}", element);
	}
}

fn for_rev(){
	for number in (1..4).rev() {
		println!("{}!", number);
	}
	println!("LIFTOFF!!!");
}

fn owner(){
	let mut s = String::from("hello");
	s.push_str(",world!");
	println!("{}",s);
}