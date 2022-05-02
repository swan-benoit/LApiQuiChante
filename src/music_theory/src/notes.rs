pub mod notes {
    pub struct Note {
        key: Key,
        alteration: Alteration,
        octave: i32,
    }

    enum Alteration {
        DoubleSharp,
        Sharp,
        Natural,
        Flat,
        DoubleFlat,
    }

    enum Key {
        C,
        D,
        E,
        F,
        G,
        A,
        B,
    }
}
