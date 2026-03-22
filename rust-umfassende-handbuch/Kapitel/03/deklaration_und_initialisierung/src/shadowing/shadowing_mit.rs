use std::io::stdin;
use std::str::FromStr;

pub fn main() {

    let mut alter = String::new();
    stdin().read_line(&mut alter);
    alter = alter.trim().to_string();

    let alter_result = i32::from_str(&alter);

    if alter_result.is_err() {
        println!(
            "{alter} ist keine Zahl. \
                Bitte eine Zahl eingeben."
        );
        return;
    }

    let alter = alter_result.unwrap();
    println!(
        "Das Alter {alter} ist: {}",
        if alter & 0x01 == 0 {
            "gerade"
        } else {
            "ungerade"
        },
    );
}