// This declaration will look for a file named `my.rs` or `my/mod.rs` and will
// insert its contents inside a module named `my` under this scope
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();
    function();
    my::indirect_access();
    my::nested::function();

    // called `my::function()`
    // called `function()`
    // called `my::indirect_access()`, that
    // > called `my::private_function()`
    // called `my::nested::function()`
}
