Several solutions exist, depending on the desired outcome:

**1. Cloning:**
```rust
fn main() {
    let mut x = 5;
    let y = x.clone(); // Create a copy
    let z = x.clone();
    y = 6;
    z = 7;
    println!("x = {}, y = {}, z = {}", x, y, z);
}
```
This creates copies of `x`, preventing the borrow checker conflict.

**2. Passing by value (if appropriate):**
```rust
fn modify(mut x: i32) -> i32 {
    x = 6;
    x
}

fn main() {
    let mut x = 5;
    x = modify(x);
    x = 7;
    println!("x = {}", x);
}
```
This avoids mutable borrows entirely.

**3. Refactoring (often the best solution):**  Consider if you actually need multiple mutable references.  Often, this points to a design issue.  Try to restructure your code to avoid this conflict.  For instance, if you need to modify `x` based on multiple conditions, create a temporary variable and then update x once.