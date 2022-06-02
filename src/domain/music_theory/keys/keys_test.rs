mod keys_test {
    use crate::domain::music_theory::keys::keys::keys::Key;

    #[test]
    fn test_next() {
        let mut key = Key::C;

        key = key.next().unwrap();
        assert_eq!(key, Key::D);

        key = key.next().unwrap();
        assert_eq!(key, Key::E);

        key = key.next().unwrap()
            .next().unwrap()
            .next().unwrap()
            .next().unwrap();
        assert_eq!(key, Key::B);

        key = key.next().unwrap()
            .next().unwrap();
        assert_eq!(key, Key::D);
    }

    #[test]
    fn test_advance() {
        let mut key = Key::C;
        key = key.nth(3).unwrap();
        assert_eq!(key, Key::F)
    }
}
