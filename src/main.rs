use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut count = 0;
    loop {
        count += 1;
        println!("\nDevine et entre un nombre entre 1 et 100!");

        let nombre_secret = rand::thread_rng().gen_range(1..101);
        let mut supposition = String::new();
        io::stdin()
            .read_line(&mut supposition)
            .expect("Échec de la lecture de l'entrée utilisateur");

        let sup = supposition.trim().parse::<i32>().unwrap();

        match sup.cmp(&nombre_secret) {
            Ordering::Less => println!(
                "Oups trop petit, c'était {}😉\nTu peux faire mieux!",
                nombre_secret
            ),
            Ordering::Greater => println!(
                "Oups trop grand, c'était {}😉\nContinue, ta du potentiel!",
                nombre_secret
            ),
            Ordering::Equal => {
                println!("T'a gagné le chackpot après {} tentatives", count);

                match count {
                    d if d >= 50 => print!("Un taux comme la mojaurité"),
                    d if d >= 11 && d < 50 => print!("Un bon taux, tu peux être fière"),
                    _ => print!("Waoow tu fais parti des top 10%, la classe😎"),
                };
                break;
            }
        }

        if count == i8::MAX {
            println!("\nTu as trop perdu Bye");
            break;
        }
    }
}
