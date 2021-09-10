use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    let mass_on_mars = cal_weight_on_mars(weight);
    println!("Mass: {}kg", mass_on_mars)
}
fn cal_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
