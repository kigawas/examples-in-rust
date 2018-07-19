pub fn run_3_1() {
    #[derive(Debug)]
    struct Person<'pl> {
        // pl for person lifetime
        name: &'pl str,
        age: u8,
    }

    #[derive(Debug)]
    struct Child {
        // remove lifetime annotation
        name: String,
        age: u8,
    }

    #[derive(Debug)]
    struct Family<'fl> {
        // fl for family lifetime
        father: Person<'fl>,
        kid: Child,
    }

    // A unit struct
    struct Nil;

    // A tuple struct
    struct Pair(i32, f32);

    // A struct with two fields
    struct Point {
        x: f32,
        y: f32,
    }

    // Structs can be reused as fields of another struct
    #[allow(dead_code)]
    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    let adam = Child {
        name: String::from("Adam"),
        age: 10,
    };
    // Print debug struct
    println!("{:?}", peter);
    println!("{:?}", adam);

    let family = Family {
        father: peter,
        kid: adam,
    };
    println!("{:?}", family);

    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let new_point = Point { x: 0.1, ..point };
    // `new_point.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    println!("my_x: {} my_y: {}", my_x, my_y);

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

pub fn run_3_2() {
    // Create an `enum` to classify a web event. Note how both
    // names and type information together specify the variant:
    // `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
    // Each is different and independent.
    enum WebEvent {
        // An `enum` may either be `unit-like`,
        PageLoad,
        PageUnload,
        // like tuple structs,
        KeyPress(char),
        Paste(String),
        // or like structures.
        Click { x: i64, y: i64 },
    }

    // A function which takes a `WebEvent` enum as an argument and
    // returns nothing.
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            // Destructure `c` from inside the `enum`.
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            // Destructure `Click` into `x` and `y`.
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}.", x, y);
            }
        }
    }

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

#[derive(Debug)]
enum Status {
    Rich,
    Poor,
}

#[derive(Debug)]
enum Work {
    Civilian,
    Soldier,
}

// Explicitly `use` each name so they are available without
// manual scoping.
use self::Status::{Poor, Rich};
// Automatically `use` each name inside `Work`.
use self::Work::*;

pub fn run_3_2_1() {
    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }

    let another_status = Rich;
    let another_work = Soldier;
    println!("status: {:?}, work: {:?}", another_status, another_work)
}

pub fn run_3_2_2() {
    // enum with implicit discriminator (starts at 0)
    enum Number {
        Zero,
        One,
        Two,
    }

    // enum with explicit discriminator
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("two is {}", Number::Two as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
    println!("grass is #{:06x}", Color::Green as i32);
}
