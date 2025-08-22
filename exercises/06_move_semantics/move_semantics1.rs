fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(88);
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics2() {
        let mut vec0 = vec![22, 44, 66];

        fill_vec(&mut vec0); // borrow instead of move

        assert_eq!(vec0, [22, 44, 66, 88]);
    }
}
