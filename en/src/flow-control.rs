// # Flow control

// ### If/else
// 1. 🌟 
// ```rust,editable

// // Fill in the blanks
// fn main() {
//     let n = 5;

//     if n < 0 {
//         println!("{} is negative", n);
//     } else if n > 0 {
//         println!("{} is positive", n);
//     } else {
//         println!("{} is zero", n);
//     }
// } 
// ```

// 2. 🌟🌟 `If/else` expression can be used in assignments.
// ```rust,editable

// // Fix the errors
// fn main() {
//     let n = 5;

//     let big_n =
//         if n < 10 && n > -10 {
//             println!(", and is a small number, increase ten-fold");
//             10 * n
//         } else {
//             println!(", and is a big number, halve the number");
//             n / 2
//         };

//     println!("{} -> {}", n, big_n);
// }
// ```

// ### For
// 3. 🌟 The `for in` construct can be used to iterate through an Iterator, e.g a range `a..b`.

// ```rust,editable

// fn main() {
//     for n in 1..=99 { // modify this line to make the code work
//         if n == 100 {
//             panic!("NEVER LET THIS RUN")
//         }
//     }

//     println!("Success!");
// } 
// ```


// 4. 🌟🌟 
// ```rust,editable

// // Fix the errors without adding or removing lines
// fn main() {
//     let names = [String::from("liming"),String::from("hanmeimei")];
//     for name in &names {
//         // Do something with name...
//     }

//     println!("{:?}", names);

//     let numbers = [1, 2, 3];
//     // The elements in numbers are Copy，so there is no move here
//     for n in numbers {
//         // Do something with n...
//     }
    
//     println!("{:?}", numbers);
// } 
// ```

// 5. 🌟
// ```rust,editable
// fn main() {
//     let a = [4, 3, 2, 1];

//     // Iterate the indexing and value in 'a'
//     for (i,v) in a.iter().enumerate() {
//         println!("The {}th element is {}",i+1,v);
//     }
// }
// ```

// ### While
// 6. 🌟🌟 The `while` keyword can be used to run a loop when a condition is true.

// ```rust,editable

// // Fill in the blanks to make the last println! work !
// fn main() {
//     // A counter variable
//     let mut n = 1;

//     // Loop while the condition is true
//     while n < 10 {
//         if n % 15 == 0 {
//             println!("fizzbuzz");
//         } else if n % 3 == 0 {
//             println!("fizz");
//         } else if n % 5 == 0 {
//             println!("buzz");
//         } else {
//             println!("{}", n);
//         }


//         n += 1;
//     }

//     println!("n reached {}, so loop is over",n);
// }
// ```

// ### Continue and break
// 7. 🌟 Use `break` to break the loop.
// ```rust,editable

// // Fill in the blank
// fn main() {
//     let mut n = 0;
//     for i in 0..=100 {
//        if n == 66 {
//            break;
//        }
//        n += 1;
//     }

//     assert_eq!(n, 66);

//     println!("Success!");
// }
// ```

// 8. 🌟🌟 `continue` will skip over the remaining code in current iteration and go to the next iteration.
// ```rust,editable

// // Fill in the blanks
// fn main() {
//     let mut n = 0;
//     for i in 0..=100 {
//        if n != 66 {
//            n+=1;
//            continue;
//        }
       
//        break;
//     }

//     assert_eq!(n, 66);

//     println!("Success!");
// }
// ```

// ### Loop 

// 9. 🌟🌟 Loop is usually used together with `break` or `continue`.

// ```rust,editable

// // Fill in the blanks
// fn main() {
//     let mut count = 0u32;

//     println!("Let's count until infinity!");

//     // Infinite loop
//     loop {
//         count += 1;

//         if count == 3 {
//             println!("three");

//             // Skip the rest of this iteration
//             continue;
//         }

//         println!("{}", count);

//         if count == 5 {
//             println!("OK, that's enough");

//             break;
//         }
//     }

//     assert_eq!(count, 5);

//     println!("Success!");
// }
// ```

// 10. 🌟🌟 Loop is an expression, so we can use it with `break` to return a value
// ```rust,editable

// // Fill in the blank
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     assert_eq!(result, 20);

//     println!("Success!");
// }
// ```

// 11. 🌟🌟🌟 It's possible to break or continue outer loops when dealing with nested loops. In these cases, the loops must be annotated with some 'label, and the label must be passed to the break/continue statement.

// ```rust,editable

// Fill in the blank
fn main() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // This would break only the inner1 loop
                break 'inner1; // `break` is also works.
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                // This breaks the outer loop
                break 'outer;
            }

            // This will continue the outer loop
            continue 'outer;
        }
    }

    assert!(count == 30);

    println!("Success!");
}
// ```

