use std::collections::HashMap;
      
pub fn freq(list: Vec<String>) -> HashMap<char, i32> {
    let mut f: HashMap<char, i32> = HashMap::new();

    for (i, char) in list.chars().enumerate() {
        f.entry(char).modify(|cnt| *cnt += 1).or_insert(1);
    }

    f
}
