///  Common modules contains all the common structures, enums and functions
mod common_modules;
/// Student module contains the task of calculating the student marks and grade
mod student;
/// Employee module contains the task of filtering the employees into 3 different categories
mod employee;


fn main() {
    println!("======== Rust Projects ========\n");

    // student::main();
    employee::main();
}