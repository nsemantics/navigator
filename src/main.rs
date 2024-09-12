use std::io;

fn main()  {
    println!("hello worldss");
    println!("guess the number");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!(" you guessed {}" , guess)
    

}