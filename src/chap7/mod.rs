#[allow(path_statements)]
#[allow(unused_must_use)]
pub fn run_7() {
    // types of statements
    // 1. variable binding
    let x = 5;

    // 2. expression + ";"
    x;

    x + 1;

    15;

    // blocks are also expressions
    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
