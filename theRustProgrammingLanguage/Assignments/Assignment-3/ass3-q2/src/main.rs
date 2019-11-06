// Q# 2. Write a Rust Program,
// 1) define a custom datatype using Struct by the name “Student”. Which contains, student’s
// name, grade, age and percentage fields (you have to set suitable data type for each field).
// 2) Create an implementation block. In Implementation block
// A. Implement an associated function by the name “new” which returns an instance of
// Student data type in main function.
// B. Implement a method which prints student percentage.
// 3) In main function:
// A. Create an instance of the above defined struct by calling associated function
// B. Print instance returned by the associated function
// C. Call method on instance returned by associated function.

// <-------struct Student start------->
#[derive(Debug)]
struct Student {
    name: String,
    grade: String,
    age: u32,
    percentage: f32,
}
// <--------struct Student end-------->

// <------ main start-------->
fn main() {
    let student_data = Student::new();
    println!("{:#?}", student_data);
    student_data.student_percentage();
}
// <-----main end-------->

// <-----implementation start-------->
impl Student {
    // <-----associated function that returns an instance of Student datatype   START   -------->
    fn new() -> Student {
        let student = Student {
            name: String::from("Arif Sanaullah"),
            grade: String::from("A+"),
            age: 22,
            percentage: 87.67,
        };
        // <-----associated function that returns an instance of Student datatype    END    -------->

        // <-----returning instance------->
        student
    }
    // <---------method that prints student percentage    START    ------->
    fn student_percentage(&self) {
        println!("Student's Percentage is: {}%.", self.percentage);
        // <---------method that prints student percentage    END    ------->
    }
}
// <-----implementation ends---------->
