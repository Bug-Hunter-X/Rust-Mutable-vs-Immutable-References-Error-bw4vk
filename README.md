# Rust Mutable vs Immutable References

This example showcases a common error when dealing with mutable and immutable references in Rust. The error arises from attempting to modify a value through an immutable reference, which violates Rust's borrowing rules designed to prevent data races and ensure memory safety.

The `bug.rs` file contains code that attempts to modify a value through both a mutable and immutable reference.  The `bugSolution.rs` file demonstrates how to correct this issue.

## Learning Points

* Understanding the difference between mutable (`&mut`) and immutable (`&`) references in Rust is crucial for preventing errors.
* Rust's borrow checker ensures that you cannot have multiple mutable references to the same value simultaneously, or have a mutable and immutable reference at the same time.
* This strictness ensures thread safety and prevents data corruption.