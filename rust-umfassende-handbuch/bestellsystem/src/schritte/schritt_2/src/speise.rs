#[derive(PartialEq, Debug, Clone)]
pub struct SpeiseId(pub String);

#[derive(PartialEq, Debug, Clone)]
pub struct Speise {
    name: SpeiseId,
}

impl Speise {
    pub fn new(name: String) -> Speise {
        Speise {
            name: SpeiseId(name),
        }
    }

    pub fn id(&self) -> SpeiseId {
        self.name.clone()
    }
}
