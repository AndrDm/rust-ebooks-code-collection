use std::num::ParseIntError;
use std::process::ExitCode;

fn main() -> Result<ExitCode, ParseIntError> {

    fn parse_eingaben() -> Result<i32, ParseIntError> {
        let eingabe_1: i32 = "123".parse::<i32>()?;
        let eingabe_2: i32 = "456".parse::<i32>()?;

        Ok(eingabe_1 + eingabe_2)
    }

    let ergebnis: i32 = parse_eingaben()?;

    // Ausgabe im Fehlerfall
    // Error: ParseIntError { kind: InvalidDigit }
    // error: process didn't exit successfully: `target\debug\result.exe` (exit code: 1)

    // Process finished with exit code 1


    Ok(ExitCode::SUCCESS)
}
