
pub fn to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.00) / 1.8000
}

pub fn to_fahrenheit(celsius: f32) -> f32 {
    celsius * 1.8000 + 32.00
}