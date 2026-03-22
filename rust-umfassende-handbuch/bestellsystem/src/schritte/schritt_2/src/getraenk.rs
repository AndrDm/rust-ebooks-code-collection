#[derive(PartialEq, Debug, Clone)]
pub struct GetraenkId(pub String);

#[derive(PartialEq, Debug, Clone)]
pub struct Getraenk {
    name: GetraenkId,
}

impl Getraenk {
    pub fn new(name: String) -> Getraenk {
        Getraenk {
            name: GetraenkId(name),
        }
    }

    pub fn id(&self) -> GetraenkId {
        self.name.clone()
    }
}
