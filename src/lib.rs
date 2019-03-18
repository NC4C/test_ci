pub mod foo {
    pub fn add(i: u8, j: u8) -> u8 {
        i + j
    }
}

#[cfg(test)]
mod tests {
    use super::foo;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_add() {
        assert_eq!(foo::add(2, 2), 4);
    }

    #[test]
    #[should_panic]
    fn test_add_overflow() {
        foo::add(128, 128);
    }
}
