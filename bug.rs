fn main() {
    let mut x = 5;
    let y = &mut x; 
    let z = &mut x; // This is the error: multiple mutable borrows
    *y = 6; 
    *z = 7;
}