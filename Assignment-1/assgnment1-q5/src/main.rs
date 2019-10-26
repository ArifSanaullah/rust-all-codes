fn main() {
    // Q # 5​ . Write a Rust program by initializing an array of Cars, and another array with their prices.
    // Print the data as below:
    // Output be like:
    // Car Name: Passo - Price: 800000
    // Car Name: Vitz -
    // Price: 900000
    // Car Name: Swift -
    // Price: 950000
    // (Note: For the sake of simplicity, limit only 5 cars.)

    let _cars = ["Suzuki", "Honda", "Toyota", "Audi", "Royals Royce"];
    let _prices = [12345, 23456, 34567, 45678, 56789];
    let mut number = 0;
    while number < _cars.len() {
        if number==4 {
            println!("Car Name: {:#?} - Price: {} ", _cars[number], _prices[number]);
            number += 1;
        }else {
            println!("Car_Name: {:#?} \t - Price: {}", _cars[number], _prices[number]);
            number += 1;
        }
        
    }
}