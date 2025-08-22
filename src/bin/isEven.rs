use rust_tut::is_even;

fn main() {
    for n in 1..=5 {
        println!("{} -> {}", n, is_even(n));
    }
}
