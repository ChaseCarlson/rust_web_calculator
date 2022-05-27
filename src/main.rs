#[macro_use] extern crate rocket;

use rust_web_calculator::endpoints::math as math_ep;

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/calc/", routes![
			math_ep::add,
			math_ep::subtract,
			math_ep::multiply,
			math_ep::divide,
			math_ep::square,
			math_ep::sqrt,
			math_ep::pow,
			math_ep::factorial
		])
}