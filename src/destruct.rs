// --- What ---
// This program demonstrates the use of some of rusts match destructuring
// capabilities. This example matches an array of four elements against match
// patterns the capture different parts of the array given the state of the
// array.
//
// Each match pattern accepts an array of at least two elements. Even though
// there are three elements in the match pattern, the "body" element acts as an
// aggregate, indicated by the "@ .." syntax. Thus it will result as an empty
// array if there are fewer than 3 elements. 
//
// The first match pattern accepts a 2+ element array whose first element is
// less than or equal to 2. The second match pattern accepts a 2+ element array
// whose first element is greater than or equal to 4. The last match pattern
// accepts any 2+ element array.

// --- Why ---
// I'd like to see if this would be an effective way of aiding in the
// implementation of a programming language interpreter after tokens have been
// produced. Several elements could be accepted by a match pattern at once rather
// than iterating over every token (e.g
//  `[Token::Number(first), Token::Add, Token::Number(second), rest @ ..] => {...}`

pub fn destruct() {
    const X: usize = 4;
    let a: [i32; X] = [3, 4, 5, 6];

    match a {
        [first @ ..=2, body @ .., last] => {
            println!(
                "First element, {first}, is less than 3. The body is {body:?}. Last is {last}"
            );
        }
        [first @ 4..=i32::MAX, body @ .., last] => {
            println!(
                "First element, {first}, is greater than 3. The body is {body:?}. Last is {last}"
            );
        }
        [first, body @ .., last] => {
            println!("First element, {first}, is 3! The body is {body:?}. Last is {last}");
        }
    }
}