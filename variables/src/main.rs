use std::io;

fn main() {
    // Static/Scalar Types: Ints, Floats, Chars, Bools

    let x: u32 = 5;
    let x: u32 = x + 1;
    {
        let x: u32 = x * 2;
        println!("The value of inner x is {x}");
    }
    println!("The value of x is {x}");

    // let mut spaces = "    ";
    // spaces = spaces.len();

    // let x = 2.0;
    // let y: f32 = 3.0;

    // let sum = 5.0 + 2.0;
    // let diff = 5.0 - 2.0;
    // let mul = 5.0 * 2.0;
    // let div = 5.0 / 2.0;
    // let trunc = -5 / 3; // -1
    // let remainder = 43 % 5;

    // let f: bool = false;
    // let z: char = 'ℤ';

    // Compound Types: Tuples, Arrays

    let tup: (i32, f64, i32) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x, y, z: {x}, {y}, {z}");
    // let five_hundred = tup.0;
    // let six_p_four = tup.1;
    // let one = tup.2;

    // let a: [i32; 4] = [1, 2, 3, 4];
    // let b: [i32; 5] = [3; 5];    // [3, 3, 3, 3, 3]
    // let two: i32 = a[1];         // 2

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Please enter an array index");

    let mut index: String = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element: i32 = a[index];

    println!("The value of the element at index {index} is: {element}");

    // Functions
    println!("Hello, world!");
    another_function(5);

    // Statement
    let y = 6;

    // Expression
    {
        let y = y + 2;
        println!("y: {y}");
    }

    // Control Flow

    let num = 3;
    if num < 4 {
        println!("true");
    } else {
        println!("false");
    }

    // if num {     // cant check bool of a number, doesn't check for existence
    if num != 0 {
        // checks if number is not 0
        println!("num is 3");
    } else if num < -1 {
        println!("Negative");
    } else {
        println!("Num: {num}");
    }

    let condition = true;
    let number = if condition { 15 } else { 26 };
    println!("number: {number}");

    // Loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result: {result}");

    // Loop Labels
    let mut count = 0;
    'counting_up: loop {
        // label outer loop
        println!("count: {count}");
        let mut remaining = 10;
        loop {
            // inner loop
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            } // break inner loop
            if count == 2 {
                break 'counting_up;
            } // break outer loop
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count: {count}");

    // While Loops - Arrays use For
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("(while) liftoff");

    // For Loops
    let a = [1, 2, 3, 4, 5];
    for el in a {
        println!("el: {el}");
    }

    // Range
    for num in (1..4).rev() {
        println!("{num}");
    }
    println!("(range) LIFTOFF");

    // Convert F to C
    // (F - 32) * 5/9
    let deg_to_cel = conv_f_to_c(100);
    println!("cel: {deg_to_cel}");

    // Fibonacci of nth number
    fib(10);

    // Twelve Days of Christmas
    // "On the {num}{suffix} of Christmas, my true love gave to me, "
    twelve_days_of_christmas();
}

fn twelve_days_of_christmas() {
    let mut num = 0;
    let mut gifts_str = String::new();
    let num_days = vec![
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth", "tenth", "eleventh", "twelveth",
    ];
    let gifts = vec![
        "a partridge in a pear tree.",
        "two turtle doves and ",
        "three french hens, ",
        "four calling birds, ",
        "five golden rings, ",
        "six geese a laying, ",
        "seven swans a swimming, ",
        "eight maids a milking, ",
        "nine ladies dancing, ",
        "ten lords a leaping, ",
        "eleven pipers piping, ",
        "twelve drummers drumming, ",
    ];

    for day in num_days {
        // append new str element to print gift_str
        gifts_str.insert_str(0, gifts[num]);
        
        // print song
        println!("On the {day} of Christmas, my true love gave to me:\n");
        println!("\t{}\n", gifts_str);
        
        // incr gift index
        num += 1;
    }

}

fn conv_f_to_c(x: i32) -> i32 {
    (x - 32) * 5 / 9
}

fn fib(x: i32) {
    if x < 1 {
        println!("0");
        return;
    };
    let mut fst = 0;
    let mut snd = 1;
    if x > 0 {
        println!("{fst}")
    };
    if x > 1 {
        println!("{snd}")
    };
    if x > 2 {
        let mut next = fst + snd;
        for num in 2..x {
            println!("{next}");
            if num + 2 <= x {
                fst = snd;
                snd = next;
                next = fst + snd;
            }
        }
    }
}
// Function definition is a Statement
fn another_function(x: i32) {
    println!("Another function fired {x}");

    let z = five();
    println!("z: {z}");

    let a = plus_one(z);
    println!("a: {a}");
}

fn five() -> i32 {
    5   // -i32 specifies the return type, can omit return keyword
}

fn plus_one(i: i32) -> i32 {
    i + 1
}
