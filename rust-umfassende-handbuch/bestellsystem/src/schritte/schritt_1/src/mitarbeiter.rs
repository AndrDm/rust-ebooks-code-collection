
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Mitarbeiter {
    id: MitarbeiterId,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct MitarbeiterId(i32);