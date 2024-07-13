use std::io;
use rand::Rng;


fn main (){
    println!("Raqamni taxmin qiling!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Yashirin raqam:  {secret_number}");

    println!("Iltimos, o'ylagan raqamingizni kiriting.");

    let mut guess = String::new(); // mutable variable

    io::stdin()
        .read_line(&mut guess)
        .expect("Qatorni o'qishda xatolik.");

    println!("Siz taxmin qilgan son: {guess}.");
}
