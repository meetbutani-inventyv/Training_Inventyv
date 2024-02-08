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


    // Creating domain array to store the letters that can be filled
    let mut dom:Vec<char> = Vec::new();
    let mut ans = String::new();
    let mut x = 0;

    for i in 0..t2.len() {
        for _j in 0..t2[i].1 {
            dom.push(t2[i].0);
        }
    }


    // Filling the missing letters in task1 with letters from the domain array
    for i in t1.chars() {
        if i=='_' && dom.len()>=x+1 {
            ans.push(dom[x]);
            x+=1;
        }
        else {
            ans.push(i);
        }
    }

    println!("\nCombined output:\n{}", ans);
}