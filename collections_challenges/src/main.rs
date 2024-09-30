use std::{
    collections::{hash_map::Entry, HashMap},
    io,
};

fn main() {
    println!("Hello, world!");
    // find_median_and_mode();
    // pig_latin();
    employee_database();
}

// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
fn find_median_and_mode() {
    let integer_vector: Vec<i32> = vec![1, 6, 4, 7, 3, 4, 8, 4, 5, 1, 11, 21, 32, 43, 54];
    let mut sorted_integer_vector: Vec<i32> = integer_vector.clone();
    sorted_integer_vector.sort_unstable();
    println!("{:?} {:?}", integer_vector, sorted_integer_vector);

    let median = sorted_integer_vector
        .get(sorted_integer_vector.len() / 2)
        .unwrap();
    println!("median: {:?}", median);

    let mut mode_map: HashMap<i32, i32> = HashMap::new();

    let mut mode: Option<i32> = None;
    let mut mode_freq: i32 = 0;

    for i in integer_vector {
        let count = mode_map.entry(i).or_insert(0);
        *count += 1;
        if count > &mut mode_freq {
            mode_freq = *count;
            mode = Some(i);
        }
    }

    println!("mode: {}", mode.unwrap());
}

// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
fn pig_latin() {
    let sample_string = "it hardly matters at all what the input string should be";

    let mut pig_latin_output: String = String::from("");

    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    sample_string.split_ascii_whitespace().for_each(|word| {
        println!("{}", word);
        let mut chars = word.chars();
        let first_char = chars.next();
        if vowels.contains(&first_char.unwrap()) {
            println!("vowel");
            pig_latin_output.push_str(word);
            pig_latin_output.push_str("-ay ");
        } else {
            println!("consonant");
            let remaining_chars: String = chars.collect();
            pig_latin_output.push_str(&remaining_chars);
            let suffix = format!("-{}ay ", first_char.unwrap());
            pig_latin_output.push_str(&suffix);
        }
    });

    println!("pig latin: {pig_latin_output}");
}

// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
fn employee_database() {
    let mut database: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Please input your command. Either \"add X to Y\" or \"get Y\"");

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        if command.starts_with("add") {
            println!("add");
            let command_parts: Vec<&str> = command.split_whitespace().collect();
            let department = command_parts[3];
            let person = command_parts[1];

            database
                .entry(String::from(department))
                .or_insert(Vec::new())
                .push(String::from(person));
        } else if command.starts_with("get") {
            println!("get");
            let command_parts: Vec<&str> = command.split_whitespace().collect();
            let department = command_parts[1];

            println!("employees in deparment {}:", department);

            match database.entry(String::from(department)) {
                Entry::Vacant(_) => {
                    println!("no employees in this department or department does not exist")
                }
                Entry::Occupied(employees) => {
                    let emplist = employees.get();
                    for ele in emplist {
                        println!("{ele}");
                    }
                }
            }
        }
    }
}
