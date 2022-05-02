mod chords {
    use crate::notes::notes::Note;

    struct Chord {
        root: Note,
        chord_type: ChordType,
    }

    // WIP
    fn get_formula(chord: ChordType) {
        match chord {
            ChordType::Min7 { .. } => {}
            ChordType::Min { .. } => {}
            ChordType::Sus4 { .. } => {}
        }
    }

    enum ChordType {
        Min7,
        Min,
        Sus4,
    }
}
