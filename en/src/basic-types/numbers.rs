// # Numbers

// ### Integer

// 1. 🌟 

// > Tips: If we don't explicitly assign a type to a variable, then the compiler will infer one for us.

// ```rust,editable

// // Remove something to make it work
// fn main() {
//     let x: i32 = 5;
//     let mut y = 5;

//     y = x;
    
//     let _z: u8 = 10; // Type of z ? 

//     println!("Success!");
// }
// ```

// 2. 🌟
// ```rust,editable

// // Fill the blank
// fn main() {
//     let v: u16 = 38_u8 as u16;

//     println!("Success!");
// }
// ```

// 3. 🌟🌟🌟  

// > Tips: If we don't explicitly assign a type to a variable, then the compiler will infer one for us.

// ```rust,editable

// // Modify `assert_eq!` to make it work
// fn main() {
//     let x = 5;
//     assert_eq!("i32".to_string(), type_of(&x));

//     println!("Success!");
// }

// // Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }
// ```

// 4. 🌟🌟 
// ```rust,editable

// // Fill the blanks to make it work
// fn main() {
//     assert_eq!(i8::MAX, 127); 
//     assert_eq!(u8::MAX, 255); 

//     println!("Success!");
// }
// ```

// 5. 🌟🌟 
// ```rust,editable

// // Fix errors and panics to make it work
// fn main() {
//    let v1 = 251_u16 + 8;
//    let v2 = u16::checked_add(251, 8).unwrap();
//    println!("{},{}",v1,v2);
// }
// ```

// 6. 🌟🌟
// ```rust,editable

// // Modify `assert!` to make it work
// fn main() {
//     let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
//     assert!(v == 1597);

//     println!("Success!");
// }
// ```


// ### Floating-Point
// 7. 🌟

// ```rust,editable

// // Fill the blank to make it work
// fn main() {
//     let x = 1_000.000_1; // ?
//     let y: f32 = 0.12; // f32
//     let z = 0.01_f64; // f64

//     assert_eq!(type_of(&x), "f64".to_string());
//     println!("Success!");
// }

// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }
// ```

// 8. 🌟🌟 Make it work in two distinct ways

// ```rust,editable

// fn main() {
//     assert!(0.1+0.2 as f32==0.3);

//     println!("Success!");
// }
// ```

// ### Range
// 9. 🌟🌟 Two goals: 1. Modify `assert!` to make it work 2. Make `println!` output: 97 - 122

// ```rust,editable
// fn main() {
//     let mut sum = 0;
//     for i in -3..2 {
//         sum += i
//     }

//     assert_eq!(sum, -5);

//     for c in 'a'..='z' {
//         println!("{}",c as u8);
//     }
// }
// ```

// 10. 🌟🌟 
// ```rust,editable

// // Fill the blanks
// use std::ops::{Range, RangeInclusive};
// fn main() {
//     assert_eq!((1..5), Range{ start: 1, end: 5 });
//     assert_eq!((1..=5), RangeInclusive::new(1, 5));

//     println!("Success!");
// }
// ```

// ### Computations

// 11. 🌟 
// ```rust,editable

// Fill the blanks and fix the errors
fn main() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1i32);
    assert!(1i32 - 2 == -1); 
    
    assert!(3 * 50 == 150);

    assert!(9.6_f32 / 3.2 == 3.0); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
// ```
