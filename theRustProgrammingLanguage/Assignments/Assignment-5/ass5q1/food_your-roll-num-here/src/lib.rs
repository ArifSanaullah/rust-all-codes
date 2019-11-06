#![allow(unused_variables)]
// Q # 1. Write a Rust program,
// Make a package with the name of ​ “food_your-roll-num-here” ​ which contain a ​ library
// crate​ . Inside the library crate (lib.rs). Create the module tree given below.
// Name of modules: ​ daily_meal,dinner,lunch,breakfast
// Name of functions: ​ func1(), func2() and func3()
mod daily_meal {
    mod dinner {}
    mod lunch {
        fn func2() {}
        fn func1() {}
    }
    mod breakfast {
        fn func3() {}
    }
}
