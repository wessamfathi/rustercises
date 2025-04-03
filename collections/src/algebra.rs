use std::collections::HashMap;
use std::io;

pub fn calc_algebra() {
    let mut vals = Vec::new();

    loop {
        println!("Enter a number to add to list or anything else to calculate");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Faied to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        vals.push(guess);
    }

    if vals.len() > 0 {
        vals.sort();
        // median is in the middle of the sorted list
        let median = vals[vals.len() / 2];

        // use a hashmap to count the mode
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut mode = 0;
        let mut highest = 0;

        for v in &vals {
            let count = map.entry(*v).or_insert(0);
            *count += 1;

            if *count > highest {
                highest = *count;
                mode = *v;
            }
        }

        println!("List: {vals:#?}\nMedian: {median}\nMode: {mode}");
    }
}
