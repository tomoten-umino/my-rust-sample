pub fn print_my_aaa(_input: i32) -> i32 {
    if _input == 1 {
        println!("test case 1");
        return 1;
    } else if _input == 2 {
        println!("test case 2");
        return 2;
    } else {
        println!("other test cases");
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn internal() {
        assert_eq!(1, print_my_aaa(1));
        assert_eq!(2, print_my_aaa(2));
        assert_eq!(-1, print_my_aaa(3));
        assert_eq!(-1, print_my_aaa(100));
    }
}
