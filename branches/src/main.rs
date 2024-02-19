// chapter end challenges
fn main() {
    println!("{}", f_to_c(70.0));

    for i in 0..50 {
        println!("{i}, {}", nth_fibonacci(i));
    }

    christmas_song();
}

fn f_to_c(temp_in_f: f64) -> f64 {
    (temp_in_f - 32.0) * 5.0 / 9.0
}

fn nth_fibonacci(n: u64) -> u64 {
    if n < 1 {
        return 0;
    };
    if n == 1 {
        return 1;
    };
    if n == 2 {
        return 1;
    };

    let mut pen = 1;
    let mut ult = 1;

    for _ in 3..n + 1 {
        let temp = pen + ult;
        pen = ult;
        ult = temp;
    }
    return ult;
}

fn christmas_song() {
    let lyrics = [
        "partridge",
        "lyric 2",
        "lyric 3",
        "lyric 4",
        "lyric 5",
        "lyric 6",
        "lyric 7",
        "lyric 8",
        "lyric 9",
        "lyric 10",
        "lyric 11",
        "lyric 12",
    ];

    for verse in 0..12 {
        println!("on day {verse}, this fella gave me: ");

        for i in (0..verse + 1).rev() {
            println!("{}", lyrics[i]);
        }
    }
}

// infinite loop
// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// ternary
// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {number}");
// }

// multiple conditions
// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// if
// fn main() {
//     let number = 7;

//     if number != 0 {
//         println!("number was something other than zero");
//     }
// }

// if else
// fn main() {
//     let number = 7;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }
