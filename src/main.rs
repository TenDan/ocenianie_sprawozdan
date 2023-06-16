use std::{io, io::Write};

enum ErrType {
    ThrowAgain(String),
    IncorrectStitches(String)
}

// Ocena za sprawozdanie
fn ocena(ilosc_oczek: u8) -> Result<&'static str, ErrType> {
    return match ilosc_oczek {
        1 => Ok("3.0"),
        2 => Ok("3.5"),
        3 => Ok("4.0"),
        4 => Ok("4.5"),
        5 => Ok("5.0"),
        6 => Err(ErrType::ThrowAgain(String::from("Wypadła 6, rzuć ponownie"))),
        _ => Err(ErrType::IncorrectStitches(String::from("Niepoprawna ilość oczek, rzuć jeszcze raz"))),
    }
}

fn main() {
    let mut ilosc_oczek = String::new();
    print!("Wprowadź liczbę oczek (od 1 do 6): ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut ilosc_oczek)
        .expect("Nie udało się pobrać ilości oczek");
    let ocena = ocena(ilosc_oczek.trim().parse::<u8>().unwrap_or_else(|x| { 
        println!("Błąd przy przetworzeniu liczby: {}", x);
        return 0;
    }));
    match ocena {
        Ok(v) => println!("Ocena ze sprawozdania: {v}"),
        Err(e) => {
            match e {
                ErrType::ThrowAgain(err) => println!("{}", err),
                ErrType::IncorrectStitches(err) => println!("{}", err)
            }
        }
    }
}
