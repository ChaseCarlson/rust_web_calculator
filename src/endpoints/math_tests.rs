use crate::endpoints::math;

#[test]
fn test_add() {
	assert_eq!(math::add(5.0, 5.0), "5 + 5 = 10");
	assert_eq!(math::add(10.0, 5.0), "10 + 5 = 15");
	assert_eq!(math::add(5.0, 10.0), "5 + 10 = 15");
}

#[test]
fn test_subtract() {
	assert_eq!(math::subtract(5.0, 5.0), "5 - 5 = 0");
	assert_eq!(math::subtract(80.0, 13.7), "80 - 13.7 = 66.3");
}

#[test]
fn test_multiply() {
	assert_eq!(math::multiply(10.0, 8.0), "10 * 8 = 80");
	assert_eq!(math::multiply(1.0, 10.0), "1 * 10 = 10");
}

#[test]
fn test_divide() {
	assert_eq!(math::divide(800.0, 10.0), "800 / 10 = 80");
	assert_eq!(math::divide(6.0, 4.0), "6 / 4 = 1.5");
	assert_eq!(math::divide(16.0, 0.0), "16 / 0 = inf");
}

#[test]
fn test_square() {
	assert_eq!(math::square(10.0), "10^2 = 100");
	assert_eq!(math::square(-2.0), "-2^2 = 4");
}

#[test]
fn test_sqrt() {
	assert_eq!(math::sqrt(25.0), "sqrt(25) = 5");
	assert_eq!(math::sqrt(-1.0), "sqrt(-1) = NaN");
}

#[test]
fn test_pow() {
	assert_eq!(math::pow(2.0,3), "2^3 = 8");
}

#[test]
fn test_factorial() {
	assert_eq!(math::factorial(8), "8! = 40320");
	assert_eq!(math::factorial(20), "20! = 2432902008176640000");
	assert_eq!(math::factorial(171), "171! = inf"); // First number we get inf for is 171.
	assert_eq!(math::factorial(99999999999999999), "99999999999999999! = inf");
	assert_eq!(math::factorial(-20), "-20! = 1") // We don't support negative factorials, so this result is acceptable
}