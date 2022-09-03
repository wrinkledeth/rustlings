// vecs1.rs
// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.
// Make me compile and pass the test!
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array

    // TODO: declare your vector here with the macro for vectors
    // Method 1: Builtin
    let v = a.to_vec();

    // Method 2: Build Vec with array
    let v = vec![10, 20, 30, 40];

    // Method 4: Iterate through array and push to vec
    let mut v: Vec<i32> = Vec::new();
    for i in &a {
        v.push(*i);
    }

    // t Method 3: Build Vec with existing awway
    // let v = vec![a];

    // println!("{:?}", v);

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
