fn main() {
    println!("{}", xid::new());
    println!("{}", xid::new_with_timestamp(0));
    println!("{}", xid::new());
    println!("{}", xid::new_with_timestamp(1717171717));
}
