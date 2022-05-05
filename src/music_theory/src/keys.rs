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

        // fn advance_by(&mut self, n: usize) -> Result<(), usize> {
        //     match self {
        //         Key::C => Result::Some(Key::D),
        //         Key::D => Result::Some(Key::E),
        //         Key::E => Result::Some(Key::F),
        //         Key::F => Result::Some(Key::G),
        //         Key::G => Result::Some(Key::A),
        //         Key::A => Result::Some(Key::B),
        //         Key::B => Result::Some(Key::C)
        //     }
        // }
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
