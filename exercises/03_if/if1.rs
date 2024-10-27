fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    let a = 10;
    let b = 20;
    // You can't directly print non strings
    // println!(bigger(a, b));
    println!("{}", bigger(a, b));

    // Primitives don't transfer ownership
    println!("{a}");
    // You can optionally experiment here.
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
