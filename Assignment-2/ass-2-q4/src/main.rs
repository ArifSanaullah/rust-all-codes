// Q # 4. Write a program to know the result of the test (Pass/Fail) by using a user defined
// function, and perform the following operations: (Note: Consider marks of only 2 subjects for
// the sake of simplicity. Maximum marks are 100 for each subject.)
// a. Add marks and print the total.
// b. Calculate the percentage and print it, return percentage to main function
// c. If percentage >= 70, Print Pass, Else, print Fail.

fn main() {
    let percentage = pass_fail(78.3, 8.7);
    if percentage >= 70.0 {
        println!("Congratulations! You have passed.");
    } else {
        println!("Sorry, You have failed.");
    }
}
fn pass_fail(subject_a: f32, subject_b: f32) -> f32{
    let total_obtained_marks: f32 = subject_a + subject_b;
    let subj_a_max_marks: f32 = 100.0;
    let subj_b_max_marks: f32 = 100.0;
    let total_max_marks: f32 = subj_a_max_marks + subj_b_max_marks;
    println!("Total obtained marks: {}", total_obtained_marks);
    let percentage: f32 = total_obtained_marks / total_max_marks * 100.0;
    println!("Obtained Percentage is: {}%.", percentage);
    return percentage;
}
