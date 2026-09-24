pub mod num;
pub mod num2;

fn main() {
    println!("{}", f64::to_bits(0_f64.next_up()));
    println!("{}", f64::to_bits((-0_f64).next_up()));
}

