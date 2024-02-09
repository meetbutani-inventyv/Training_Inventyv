///  Common modules contains all the common structures, enums and functions
mod common_modules;
/// Student module contains the task of calculating the student marks and grade
mod student;
/// Employee module contains the task of filtering the employees into 3 different categories
mod employee;
/// String_task module contains the task related to string
mod string_task;
/// This module contains the student and employee task's implementation using hashmap
mod hashmap;

fn main() {
    println!("======== Rust Projects ========\n");

    // student::main();
    // employee::main();.
    // string_task::main();
    // hashmap::student::main();
    hashmap::employee::main();
}