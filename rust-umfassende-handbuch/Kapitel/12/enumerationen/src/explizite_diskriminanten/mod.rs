mod mit_literalen {
    enum HttpStatusCodes {
        Ok = 200,
        Created = 201,
        // ...
        NotFound = 404,
        // ...
        InternalServerError = 500,
    }
}

mod mit_literalen_gemischt {
    enum HttpStatusCodes {
        Ok = 200,
        Created, // 201
        // ...
        NotFound = 404,
        MethodNotAllowed, // 405
        // ...
        InternalServerError = 500,
        NotImplemented, // 501
        // ...
    }
}

mod mit_konstanten {
    const OK_WERT: isize = 200;
    static NOT_FOUND_WERT: isize = 404;

    const fn berechne_code() -> isize {
        // ...
        499 + 1
    }

    pub enum HttpStatusCodes {
        Ok = OK_WERT,
        Created = 201,
        // ...
        // Fehler, der Wert muss konstant sein
        // NOT_FOUND_WERT ist statisch!
        // NotFound = NOT_FOUND_WERT,
        // ...
        InternalServerError = berechne_code(),
    }
}



enum Farben {
    Keine = -1,
    Rot, // 0
    Gruen = 3,
    Blau, // 4
    Reserve = -200,
    Reserve2, // -199, ...
}

pub fn main() {

    println!("{}", mit_konstanten::HttpStatusCodes::InternalServerError as isize);
}
