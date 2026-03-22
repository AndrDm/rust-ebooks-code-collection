enum Form {
    Punkt { x: i32, y: i32 },
    // Fehler, Self::Punkt ist kein Typ, sondern eine Variante!
    // Linie { a: Form::Punkt, b: Form::Punkt },
}

mod form {
    struct Punkt {
        x: i32,
        y: i32,
    }
    struct Linie {
        a: Punkt,
        b: Punkt,
    }
}
