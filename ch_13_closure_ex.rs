use std::collections::HashMap;

pub fn get_freq(nums: Vec<i32>) -> HashMap<i32, i32> {
    let mut mapper: HashMap<i32, i32> = HashMap::new();

    nums
        .iter()
        .for_each(
            |&num| { *mapper.entry(num).or_insert_with(|| 0) += 1; }
        );

    mapper
}

pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut countA = 0;
    let mut countB = 0;
    let mapA = get_freq(nums1);
    let mapB = get_freq(nums2);

    for (num, amount) in &mapA {
        if let Some(value) = mapB.get(num) {
            countA += amount;
            countB += *value;
        };
    }
    vec![countA, countB]
}
