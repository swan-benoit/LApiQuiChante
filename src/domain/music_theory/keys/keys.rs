pub mod keys {
    #[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
    pub enum Key {
        C,
        D,
        E,
        F,
        G,
        A,
        B,
    }

    impl Iterator for Key {
        type Item = Key;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Key::C => Option::Some(Key::D),
                Key::D => Option::Some(Key::E),
                Key::E => Option::Some(Key::F),
                Key::F => Option::Some(Key::G),
                Key::G => Option::Some(Key::A),
                Key::A => Option::Some(Key::B),
                Key::B => Option::Some(Key::C)
            }
        }

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            (1..=n).fold(
                Some(self.clone()),
                |key, _| { key.unwrap().next() },
            )
        }
    }

    pub fn get_key(posistion: i32) -> Result<Key, &'static str> {
        match posistion {
            0 => Ok(Key::C),
            2 => Ok(Key::D),
            4 => Ok(Key::E),
            5 => Ok(Key::F),
            7 => Ok(Key::G),
            9 => Ok(Key::A),
            11 => Ok(Key::B),
            _ => Err("No notes match the provided position")
        }
    }
}
