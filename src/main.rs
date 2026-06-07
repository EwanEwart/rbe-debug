// `Derive` 
// the `fmt::Debug` implementation  for `Structure`. 
// `Structure` is a structure, 
// which contains a single `i32`.
#[derive(Debug)]
#[allow(dead_code)] // suppress warnings about dead code.
struct Structure(i32);

// println!("Now {:?} will print!", Structure(3));

// Put a `Structure` inside of the structure `Deep`.  
// Make it printable, too.
#[derive(Debug)]
#[allow(dead_code)] // suppress warnings about dead code.
struct Deep(Structure);

// println!("Now {:?} will print!", Deep(Structure(7)).0.0);


fn main() {
    // Printing with `{:?}` is similar to printing with `{}`.
    println!("(1) {:?} months in a year.", 12);

    println!("(2) {1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("(3) Now {:?} will print!", Structure(3));

    // The problem with `derive` is  
    // there is no control over what the results look like.
    // What if I want this to just show a `7`?
    println!("(4) Now {:?} will print!", Deep(Structure(7)));


    // Rust also provides “pretty printing” with {:#?}.
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print:
    // {:#?}
    #[derive(Debug)]
    #[allow(dead_code)] // suppress warnings about dead code.
    struct Person<'a> {
        name: &'a str,
        age: u8
    }
    println!("(5) {:#?}", peter); // Pretty print with {:#?}
    /*
    One can manually implement `fmt::Display` to control the display.
    
    See also:
    attributes, derive, std::fmt, and struct
 */
}
