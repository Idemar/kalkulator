use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let første: String = args.nth(1).unwrap();
    let operatør: char = args.nth(0).unwrap().chars().next().unwrap();
    let andre: String = args.nth(0).unwrap();

    let første_nummer = første.parse::<f32>().unwrap();
    let andre_nummer = andre.parse::<f32>().unwrap();
    let resultat = operasjoner(operatør, første_nummer, andre_nummer);

    println!(
        "{}",
        utdata(første_nummer, operatør, andre_nummer, resultat)
    );
}

fn utdata(første_nummer: f32, operatør: char, andre_nummer: f32, resultat: f32) -> String {
    format!(
        "{} {} {} = {}",
        første_nummer, operatør, andre_nummer, resultat
    )
}

fn operasjoner(operatør: char, første_nummer: f32, andre_nummer: f32) -> f32 {
    match operatør {
        '+' => første_nummer + andre_nummer,
        '-' => første_nummer - andre_nummer,
        '/' => første_nummer / andre_nummer,
        '*' | 'X' | 'x' => første_nummer * andre_nummer,
        _ => panic!("Ugyldig operatør brukt."),
    }
}
