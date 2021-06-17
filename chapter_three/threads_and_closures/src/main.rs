fn main() {
    let example_closure: fn(&str) = |string_input: &str| {
        println!("{}", string_input);
    };
    example_closure("this is a closure");

    {
        let base_rate: f32 = 0.03;
        let calculate_interest = |loan_amount: &f32| {
            return loan_amount * &base_rate
        };
        println!("the total interest to be paid is: {}", 
                 calculate_interest(&32567.6));
    }
}
