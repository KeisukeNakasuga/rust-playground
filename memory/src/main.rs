fn main() {
    let hex1: u32 = 0xDEADBEEF;
    println!("n1: {:p}", &hex1);
    println!("n1 size: {:?} bytes", std::mem::size_of_val(&hex1));
    println!("STOP");
}
