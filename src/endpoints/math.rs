use rocket::*;

#[get("/add/<x>/<y>")]
pub fn add(x: f64, y: f64) -> String {
	let res: f64 = x + y;
	format!("{} + {} = {}", x, y, res)
}

#[get("/subtract/<x>/<y>")]
pub fn subtract(x: f64, y: f64) -> String {
	let res: f64 = x - y;
	format!("{} - {} = {}", x, y, res)
}

#[get("/multiply/<x>/<y>")]
pub fn multiply(x: f64, y: f64) -> String {
	let res: f64 = x * y;
	format!("{} * {} = {}", x, y, res)
}

#[get("/divide/<x>/<y>")]
pub fn divide(x: f64, y: f64) -> String {
	let res: f64 = x / y;
	format!("{} / {} = {}", x, y, res)
}

#[get("/square/<x>")]
pub fn square(x: f64) -> String {
	let res: f64 = x.powi(2);
	format!("{}^2 = {}", x, res)
}

#[get("/sqrt/<x>")]
pub fn sqrt(x: f64) -> String {
	let res: f64 = x.sqrt();
	format!("sqrt({}) = {}", x, res)
}

#[get("/pow/<x>/<y>")]
pub fn pow(x: f64, y:i32) -> String {
	let res: f64 = x.powi(y);
	format!("{}^{} = {}", x, y, res)
}

#[get("/factorial/<x>")]
pub fn factorial(x: i64) -> String {
	let mut res: f64 = 1.0;
	for i in 1..x+1 {
		if res == std::f64::INFINITY {
			break; // Once we reach infinity, we aren't going to get any different of a value. Stop the loop so we don't waste resources computing the factorial.
		}
		res *= i as f64;
	}
	format!("{}! = {}", x, res)
}