pub fn main() -> Vec<(char, u32)> {
    let str1 = "lorem Ipsum";
    let str2 = "Ceaser Cypher";

    // Defining 3 empty vectors to store the matched chars, unmatched chars and the resultant chars along with their frequencies 
    let mut matched:Vec<(char, u32)> = Vec::new();
    let mut unmatched:Vec<(char, u32)> = Vec::new();
    let mut result:Vec<(char, u32)> = Vec::new();


    // Finding the matched and the unmatched chars from both the strings with their corresponding frequencies
    for c in str1.chars() {
        if str2.contains(c) && !check_presence(&matched, c) && c!=' ' {
            count_and_add(str1, str2, c,  &mut matched);
        }
        else if !matched.contains(&(c,0)) && !unmatched.contains(&(c,0)) && c!=' ' {
            unmatched.push((c,0));
        }
    }
    
    for c in str2.chars() {
        if !str1.contains(c) && !unmatched.contains(&(c,0)) && c!=' ' {
            unmatched.push((c,0));
        }
    }


    // Combining the matched and the unmatched vectors along with their frequencies
    result.extend(matched);
    result.extend(unmatched);


    // Sorting the result vector alpabetically
    sort_vector(&mut result);

    return result;
}


/// Function to check whether the given character is already present in the matched list or not
fn check_presence(matched:&Vec<(char, u32)>, ch:char) -> bool {
    for i in matched {
        if i.0 == ch {
            return true;
        }
    }
    return false;
}


/// Function to count the frequency of the matched characters from both strings
fn count_and_add(s1:&str, s2:&str, ch:char, matched:&mut Vec<(char, u32)>) {
    let mut count = 0;

    for i in s1.chars() {
        if i == ch {
            count+=1;
        }
    }

    for i in s2.chars() {
        if i == ch {
            count+=1;
        }
    }

    matched.push((ch, count));
}


/// Function to sort the vector consisting of "character: count" pairs
fn sort_vector(chars: &mut Vec<(char, u32)>) {
    let len = chars.len();

    for _i in 0..len {
        for j in 0..len - 1 {
            if chars[j].0 > chars[j+1].0 {
                let temp = chars[j];
                chars[j] = chars[j+1];
                chars[j+1] = temp;
            }
        }
    }
}