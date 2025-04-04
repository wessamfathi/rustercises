mod algebra;
mod data_entry;
mod pig_latin;

fn main() {
    println!("Hello, world!");

    data_entry::enter_data();

    algebra::calc_algebra();

    println!("----------------------------");

    pig_latin::pig_latinify();
}
