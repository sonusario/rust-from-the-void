// --- What ---
// This program demonstrates the use of dynamic closures to mutate an
// Environment struct. In this instance the Environment just contains a
// vector of i32s, but it could be anything. The closure for "f" adds 1 to each
// number in the vector. The closure for "g" append 0 to the vector.

// --- Why ---
// I would like to use this to write an interpreter for a programming language
// I am working on. When the user defines a function, the interpreter will store
// a closure of that function which will mutate the environment when called.

pub fn dyn_mut_closures() {
    let f = Function {
        closure: &mut |env: &mut Environment| {
            env.nums.iter_mut().for_each(|x| *x += 1);
        },
    };

    let g = Function {
        closure: &mut |env: &mut Environment| {
            env.nums.push(0);
        },
    };

    let mut env = Environment::new([1, 2, 3, 4, 5]);
    (f.closure)(&mut env);
    (g.closure)(&mut env);

    println!("{env:?}");
}

#[derive(Debug)]
struct Environment {
    nums: Vec<i32>,
}

impl Environment {
    fn new(arr: [i32; 5]) -> Environment {
        Environment {
            nums: Vec::from(arr),
        }
    }
}

struct Function<'a> {
    closure: &'a mut dyn FnMut(&mut Environment),
}