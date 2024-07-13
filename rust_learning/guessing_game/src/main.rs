use std::io;

fn main (){
    println!("Raqamni taxmin qiling!");

    println!("Iltimos, o'ylagan raqamingizni kiriting.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Siz taxmin qilgan son: {guess}.");
}
