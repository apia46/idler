pub mod num;

fn main() {
    println!("{}", f64::to_bits(0_f64.next_up()));
    println!("{}", f64::to_bits((-0_f64).next_up()));
}

