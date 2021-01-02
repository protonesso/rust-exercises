struct Mystruct {
	name: String,
	age: i32
}

struct Ourstruct {
	name: String,
	age: i32
}

pub fn structs() {
	let name = String::from("Shibobu");
	let age = 16;
	let mystruct2 = Mystruct {name, age};

	let ourstruct2 = Ourstruct {name: "Shinobu".to_string(), age: 16};

	println!("{} {} {} {}", mystruct2.name, mystruct2.age, ourstruct2.name, ourstruct2.age);
}
