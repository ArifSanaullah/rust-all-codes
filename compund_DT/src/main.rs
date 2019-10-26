fn main() {
    let student: (u32,char,f64) = (25, 'A', 80.5);
    println!("{}",student.0);
    println!("{}",student.1);
    println!("{}",student.2);
    // println!("{}",student);

    println!("Desctructuring");

    let (x,y,z) = student;

    println!("{}",x);
    println!("{}",y);
    println!("{}",z);

}
