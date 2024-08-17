fn calculator_bmi(weight: f32, height: f32) -> f32 {
    weight / (height * height)
}

fn main() {
    println!("Welcome to the BMI Calculator!");

    let weight = 66.8; // in kg
    let height = 1.748; // in m

    let bmi = calculator_bmi(weight, height);

    println!("Your BMI is: {:.3}", bmi);
}
