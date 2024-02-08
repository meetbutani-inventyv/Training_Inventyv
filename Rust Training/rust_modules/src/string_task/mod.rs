/// Task_1 module contains the task of splitting the string by using the given deliminator
pub mod task_1;
/// Task_2 module contains the task of finding the character frequencies on 2 given strings
pub mod task_2;

pub fn main() {
    // Fetching the output from task1 and task2 modules and storing it in t1 and t2 variables
    let t1 = task_1::main();
    let t2 = task_2::main();

    println!("Task-1 output:\n{}", t1);
    println!("\nTask-2 output:\n {:?}", t2);
}