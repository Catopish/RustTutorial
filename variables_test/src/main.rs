fn main() {
    //Stuff like this is called shadowing,
    let x = 5;
    println!("The value of x is: {x}");
    {
        let x = (x + 10) * 10;
        println!("The value of x is: {x}");
    }
    let x = 6;
    println!("The value of x is: {x}");

    //Stuff like this call mutating
    let mut y = 50;
    println!("The value of y is: {y}");
    y = 612;
    println!("The value of y is: {y}");

    //Constants, must be use all caps, and underscore between words
    const SECRET_NUMBERS: u32 = 1010;
    println!("The secret number is {SECRET_NUMBERS}");

    //tuple
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    //calling tuple using .<index>
    println!(
        "\nfive_hundred = {}, six_point_four={}, one={}",
        five_hundred, six_point_four, one
    );

    //caling tuple using indexing to variables
    let (i, j, k) = x;
    println!("\nfirst = {i}, second={j}, third={k}");

    //array
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let mut num = 1;
    for x in months {
        println!("{x} is the {num}th Months");
        num += 1;
    }
}
