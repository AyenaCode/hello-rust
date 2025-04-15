use rand::Rng;
use std::io;
fn main() {
    let mut count = 0;
    loop {
        count += 1;
        println!("Devinez le nombre !");
        println!("Veuillez entrer un nombre.");

        let nombre_secret = rand::thread_rng().gen_range(1..101);
        let mut supposition = String::new();
        io::stdin()
            .read_line(&mut supposition)
            .expect("Échec de la lecture de l'entrée utilisateur");

        let sup = supposition.trim().parse::<i32>().unwrap();

        if sup == nombre_secret {
            println!(
                "Bravo, après {}, Vous avez gagnez le jackpot 😁😁, c'est bien {} ",
                count, sup
            );
            break;
        }
        println!("Votre nombre : {}", sup);
        println!("Le nombre secre est : {}", nombre_secret);
    }
}
