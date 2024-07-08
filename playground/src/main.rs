use std::collections::vec_deque::VecDeque;
use std::fmt::Debug;

fn main() {
	// let mut vec:Vec<Box<dyn Debug>> =Vec::new();
	//
	// let a:Box<dyn Debug> = Box::new(100);
	// vec.push(a);
	//
	// for elem in vec.iter() {
	// 	println!("{elem:?}")
	// }

	some_use_box(100);
}

fn some_use_box(b:impl Debug){
	println!("{b:?}")
}
