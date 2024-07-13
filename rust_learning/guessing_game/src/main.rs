use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main () {
    println!("Raqamni taxmin qiling!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Yashirin raqam:  {secret_number}");

    loop {
        println!("Iltimos, o'ylagan raqamingizni kiriting.");

        let mut guess = String::new(); // mutable variable

        io::stdin()
            .read_line(&mut guess)
            .expect("Qatorni o'qishda xatolik.");

        let guess: u32 = guess.trim().parse().expect("Iltimos, son kiriting.");

        println!("Siz taxmin qilgan son: {guess}.");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Kichikroq son o'yladiz!"),
            Ordering::Greater => println!("Kattaroq son o'yladiz!"),
            Ordering::Equal => { 
                println!("Tabriklaymiz. To'g'ri raqamni topdingiz!");
                break;
            }
        }    
    }
}