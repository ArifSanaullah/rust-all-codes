use std::collections::HashMap;
fn main() {
    
    let mut map1 = HashMap::new();


    // insert values in hashmap
    map1.insert(String::from("Blue"), 10);
    map1.insert(String::from("Green"), 15);
    println!("{:?}",map1);


    // constructing HashMap using collect() method
    // remember...! all the keys and values in a hashmap must contain a homogenous datatype
    let teams = vec![String::from("Blue"), String::from("Green"), String::from("Yellow"), String::from("White")];
    let scores = vec![10, 20, 30, 50, 30];

    let map2: HashMap<_,_> = teams.iter().zip(scores.iter()).collect();
    println!("{:#?}",map2);


    // ownership and  HashMaps
    let mut map3 = HashMap::new();
    let var_key = String::from("fav_color");
    let var_value = String::from("Pink");
    // remember...! insert function always takes the ownership of variables/arguments passed to it.
    map3.insert(var_key, var_value);
    println!("{:?}", map3);
    // println!("{}", var_value);


    // accessing values in hashmaps
    let result = map3.get(&String::from("fav_color"));
    println!("{:?}",result);


    // accessing values in HM through for loop
    for (key,value) in &map2 {
        println!("\n{}, {}", key, value);
    }


    // overwriting keys in HM
    // let mut map4: HashMap<_,_> = teams.iter().zip(scores.iter()).collect();
    let mut map4 = HashMap::new();
    map4.insert(String::from("Green"), 20);
    map4.insert(String::from("Blue"), 30);
    println!("{:?}",map4);
    let over_key = String::from("Green");
    let over_value = 25;
    map4.insert(over_key, over_value);
    println!("\n {:?}",map4);


    // storing values on basis of previous values
    let mut map5 = HashMap::new();
    map5.insert(String::from("Blue"), 22);
    map5.entry(String::from("Yellow")).or_insert(34);
    map5.entry(String::from("Blue")).or_insert(500);
    println!("{:?}",map5);


    // updating a value based on old value
    let mut map6 = HashMap::new();
    let str_var = "Hello world Wonderful world";
    for word in str_var.split_whitespace() {
        let count = map6.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}",map6);
}
