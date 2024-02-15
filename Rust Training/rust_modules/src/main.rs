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
/// This module contains the table task implementation using lazy_static
mod lazy_static;
/// This module contains the tasks related to threads in rust
mod thread_task; 
/// Config module to understand the config in rust
mod config;
/// This module contains the basic Rest API's for student, employee & users data
mod basic_crud_api;


fn main() {
    println!("======== Rust Projects ========\n");

    // student::main();
    // employee::main();.
    // string_task::main();
    // hashmap::student::main();
    // hashmap::employee::main();
    // lazy_static::main();
    // thread_task::main();
    // config::main();
    basic_crud_api::main();
}