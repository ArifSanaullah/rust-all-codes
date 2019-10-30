fn main() {
    {
        // method one - creating a vector
        let mut v1 = vec![10, 20, 30];
        println!("{:#?}", v1);
        // method two - creating an empty vector
        let v2: Vec<i32> = Vec::new();
        // while creating an empty vector it is necessary to define datatype of vector explicitly.

        // pushing and popping values in vectors
        v1.push(60);
        println!("{:#?}", v1);
        v1.pop();
    }
    // vector drops after scope ends
    // println!("{:?}", v1);

    // accessing values of vectors
    let v3 = vec![2, 4, 6, 8, 10];
    let element: u32 = v3[2];
    let element0 = &v3[0];
    println!("{:?}", element);
    println!("{:?}", &v3[3]);
    println!("{:?}", element0);
    let element1 = v3.get(1);
    println!("{:?}", element1);
    match element1 {
        Some(value) => println!("{}", value),
        None => println!("Nothing"),
    }

    // accessing invalid index in a vector
    let mut v4 = vec![1, 3, 5, 7, 9];
    // let v4e6 = v4[6];
    // println!("{:?}",v4e6);
    let v4e7get = v4.get(7);
    println!("from get {:?}", v4e7get);

    // what to avoid in a vector
    let mut v5 = vec![12, 23, 34, 45, 56];
    let v5e0 = &v5[0];
    v5.push(67);

    // looping through vectors
    // i) always pass refrence of vectors while doing any process on it.
    for i in &v5 {
        println!("from for loop {:?}", i);
    }
    // looping through vectors and changing it's values
    for i in &mut v4{
        *i = *i*2;
        println!("{:?}", i);
    }


    // using enum to store multiple datatypes
    let v6 = vec![SpreadSheetCell::Int(10), SpreadSheetCell::Float(10.4), SpreadSheetCell::Text(String::from("Hello world...!"))];
    println!("{:?}",v6);

    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
}

// vectores are just like arrays except arrays are static in size but vectors are not size dependent. size of a vector can be changed at any time. vector takes homogenous datatypes of each element present in them.
