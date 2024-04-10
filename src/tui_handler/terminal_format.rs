use std::io::{self, Write};



/// Clears `n` lines from the terminal
pub fn clear(n: u16) {
    // Move the cursor up `n` times
    for _ in 0..n {
        print!("\x1B[1A"); // Move up one line
        print!("\x1B[K");  // Clear the line
    }

    // Move the cursor back to the beginning of the last line
    print!("\r");

    // Flush stdout to ensure the output is visible immediately
    io::stdout().flush().unwrap();
}

fn main() {
    clear(5); // Example usage: Clear 5 lines
}

