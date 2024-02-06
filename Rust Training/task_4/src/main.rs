fn main() {
    let str1 = "hello world";
    let str2 = "inventyv software";
    
    let mut matched:Vec<(char, u32)> = Vec::new();
    let mut unmatched:Vec<(char, u32)> = Vec::new();
    let mut result:Vec<(char, u32)> = Vec::new();


    for c in str1.chars() {
        if str2.contains(c) && !matched.contains(&(c,0)) && c!=' ' {
            // matched.push((c,0));
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

    println!("Matched: {:?}", matched);
    println!("Unmatched: {:?}", unmatched);

    result.extend(matched);
    result.extend(unmatched);
    // result.sort();

    result.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));

    println!("\nResult: {:?}", result);
}


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


fn sortVector(chars: &mut Vec<char>) {
    let len = chars.len();

    for i in 0..len {
        for j in 0..len - 1 {
            if chars[j] > chars[j+1] {
                let temp = chars[j];
                chars[j] = chars[j+1];
                chars[j+1] = temp;
            }
        }
    }
}