pub fn main() -> String{ 
    let s = "Welcome to Inventyv Software services";
    let delim = 'e';
    let mut new_s:String = String::new();

    // for i in 0..s.len() {
    //     if &s[i..i+1] == delim {
    //         print!("{}_", &s[x..i]);
    //         x = i+1;
    //     }
    // }


    // Replacing the deliminator in the given string with "_"
    for i in s.chars() {
        if i == delim {
            new_s += "_";
        }
        else {
            new_s += &String::from(i);
        }
    }

    return new_s
}
