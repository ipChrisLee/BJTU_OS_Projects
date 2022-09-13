use std::any::type_name;

pub fn print_type_name<T>(_: &T) {
	println!("{}", std::any::type_name::<T>());
}