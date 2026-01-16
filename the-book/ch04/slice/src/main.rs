fn main() {
    // ============================================================
    // RUST SLICES - A Complete Beginner's Guide
    // ============================================================
    //
    // If you're used to JavaScript: array.slice(0, 2)
    // In Rust it's:                 &array[0..2]
    //
    // Key difference: Rust slices are REFERENCES (borrows), not copies!
    // The '&' means "I'm borrowing a view into this data"
    // ============================================================

    println!("=== PART 1: ARRAY SLICES ===\n");

    // Create a simple array of numbers
    let numbers = [10, 20, 30, 40, 50];
    //             ^   ^   ^   ^   ^
    //  index:     0   1   2   3   4

    // ---------------------------------------------------------
    // Basic slice syntax: &array[start..end]
    // - 'start' is INCLUSIVE (included in the slice)
    // - 'end' is EXCLUSIVE (NOT included in the slice)
    // - This is just like Python's list[0:2] or JS's slice(0, 2)
    // ---------------------------------------------------------

    // Get elements at index 1 and 2 (i.e., 20 and 30)
    let slice1 = &numbers[1..3]; // start=1, end=3 (so indexes 1,2)
    println!("numbers[1..3] = {:?}", slice1); // [20, 30]

    // Get the first two elements (index 0 and 1)
    let slice2 = &numbers[0..2];
    println!("numbers[0..2] = {:?}", slice2); // [10, 20]

    // ---------------------------------------------------------
    // Shorthand syntax - you can omit start or end
    // ---------------------------------------------------------

    // If starting from 0, you can omit it:
    let slice3 = &numbers[..2]; // same as [0..2]
    println!("numbers[..2]  = {:?}", slice3); // [10, 20]

    // If going to the end, you can omit the end:
    let slice4 = &numbers[2..]; // from index 2 to the end
    println!("numbers[2..]  = {:?}", slice4); // [30, 40, 50]

    // If you want the whole thing:
    let slice5 = &numbers[..]; // same as [0..5] or the whole array
    println!("numbers[..]   = {:?}", slice5); // [10, 20, 30, 40, 50]

    println!("\n=== PART 2: STRING SLICES ===\n");

    // ---------------------------------------------------------
    // String slices work the same way, but with bytes/characters
    // ---------------------------------------------------------

    let greeting = String::from("Hello, World!");
    //                           ^^^^^  ^^^^^^
    //                           01234  567890...
    // Index:                    H=0, e=1, l=2, l=3, o=4, ,=5, space=6, W=7...

    // Get "Hello" (characters 0-4, so we use 0..5)
    let hello = &greeting[0..5];
    println!("greeting[0..5] = {}", hello); // "Hello"

    // Get "World" (characters 7-11, so we use 7..12)
    let world = &greeting[7..12];
    println!("greeting[7..12] = {}", world); // "World"

    // Using the shorthand:
    let hello_short = &greeting[..5]; // same as [0..5]
    println!("greeting[..5] = {}", hello_short); // "Hello"

    println!("\n=== PART 3: WHY SLICES MATTER (Safety!) ===\n");

    // ---------------------------------------------------------
    // The magic of slices: they're tied to the original data!
    // Rust won't let you mess up and use invalid data.
    // ---------------------------------------------------------

    let text = String::from("hello world");

    // Get a slice of "hello"
    let word = &text[0..5];
    println!("The word is: {}", word);

    // WHY THIS MATTERS:
    // If 'text' were mutable and we tried to call text.clear() here,
    // Rust would REFUSE to compile! You can't modify data while
    // something else (like 'word') is borrowing it.
    //
    // Try this yourself: add 'mut' to 'text' above, then uncomment:
    // text.clear(); // Won't compile! 'word' is still borrowing 'text'

    println!("Still valid: {}", word);

    println!("\n=== PART 4: THE &str TYPE ===\n");

    // ---------------------------------------------------------
    // &str (pronounced "string slice") is the type of string slices
    // ---------------------------------------------------------

    // This is a String (owned, heap-allocated, can grow)
    let owned_string: String = String::from("I own this data");

    // This is a &str (borrowed slice, just a view into some string data)
    let slice_of_string: &str = &owned_string[0..4]; // "I ow"

    // String literals are ALSO &str (they're slices into the binary!)
    let literal: &str = "I'm a string literal";

    println!("owned_string: {}", owned_string);
    println!("slice_of_string: {}", slice_of_string);
    println!("literal: {}", literal);

    println!("\n=== PART 5: PRACTICAL EXAMPLE ===\n");

    // ---------------------------------------------------------
    // A function that finds the first word in a string
    // ---------------------------------------------------------

    let sentence = "The quick brown fox";
    let first = first_word(sentence);
    println!("First word of '{}' is '{}'", sentence, first);

    let another = "Rust";
    let first2 = first_word(another);
    println!("First word of '{}' is '{}'", another, first2);

    println!("\n=== QUICK REFERENCE ===\n");
    println!("&arr[0..3]  -> elements 0, 1, 2");
    println!("&arr[..3]   -> elements 0, 1, 2 (start omitted = 0)");
    println!("&arr[2..]   -> element 2 to end");
    println!("&arr[..]    -> entire array/string");
    println!("\nRemember: start is INCLUSIVE, end is EXCLUSIVE!");
}

/// Finds the first word in a string (everything before the first space).
/// If there's no space, returns the whole string.
///
/// - Takes a &str (string slice) as input
/// - Returns a &str (string slice) pointing to the first word
fn first_word(s: &str) -> &str {
    // Convert string to bytes so we can iterate
    let bytes = s.as_bytes();

    // Loop through each byte with its index
    for (index, &byte) in bytes.iter().enumerate() {
        // b' ' is the byte literal for a space character
        if byte == b' ' {
            // Found a space! Return slice from start to here
            return &s[0..index];
        }
    }

    // No space found, return the whole string
    &s[..]
}
