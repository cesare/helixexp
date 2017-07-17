#[macro_use]
extern crate helix;

ruby! {
    class Helixexp {
        def greet(name: String) {
            println!("hello, {}", name);
        }
    }
}
