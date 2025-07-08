fn main() {
    //https://doc.rust-lang.org/rust-by-example/hello/print.html

    println!("{} days", 31); // {} gets automatically replaced with any argument (will be stringified)

    //Another example for positional argument with integer inside {}

    println!("{0}, Do you want {1} or do you want {2}", "Samaksh", "Butter Chicken", "Chole Bhature");

    // Another example as Named Argument

    println!("My name is {name}. I am a {job}. I love to {hobby}.",
            name = "Samaksh",
            job = "Bioinformatician",
            hobby = "Sing");


    //Invoking different formats by specifying format character.

    println!("Base 10:              {}", 581499); 
    println!("Base 2 (binary):      {:b}", 581499);
    println!("Base 8 (octal):       {:o}", 581499);
    println!("Base 16 (hexadecimal  {:x}", 581499);
    println!("Base 16 (hexadecimal  {:X}", 581499);

    // Padding a number with whitespaces
    println!("{number:>8}", number = 8); //adding 7 whitespaces before number 8

    // Pad numbers with extra zeroes
    println!("{number:0>8}", number = 8); // adding 7 zeroes before number
    println!("{number:0<8}", number = 8); // adding 7 zeroes after number


    // using named argument in format specifier by appending '$'

    println!("{number:0>width$}", number = 6, width = 5);

    println!("{number:0<width$}", number = 6, width = 5);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
