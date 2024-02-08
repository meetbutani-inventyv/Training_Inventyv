///  Common modules contains all the common structures, enums and functions
mod common_modules;

/// Student module contains the task of calculating the student marks and grade
mod student;


fn main() {
    println!("======== Rust Projects ========\n");

    student::main();
}