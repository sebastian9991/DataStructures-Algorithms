use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations_sorted = citations.clone();
        let mut map_counter = HashMap::new();
        citations_sorted.sort_by(|a, b| b.cmp(a));

        let mut count;
        if citations[0] == 0 {
            count = 0;
        } else {
            count = 1;
        }
        for cit_value in citations_sorted {
            if !map_counter.contains_key(&cit_value) {
                map_counter.insert(cit_value, count);
            } else {
                match map_counter.get_mut(&cit_value) {
                    Some(v) => *v = count,
                    None => (),
                }
            }
            count += 1;
        }

        map_counter.retain(|k, v| k <= v);
        let (max_key, max_value);
        if !map_counter.is_empty() {
            (max_key, max_value) = map_counter.into_iter().max_by_key(|entry| entry.0).unwrap();
        } else {
            (max_key, max_value) = (1, 0);
        }

        if max_key == 0 && max_value > 0 {
            return 1;
        }
        max_key
    }
}

fn main() {
    let cit = vec![3, 0, 6, 1, 5];
    let cit1 = vec![1, 3, 1];
    let cit2 = vec![5, 4, 5, 3, 5];
    let cit_edge = vec![0, 3, 0, 0];
    let cit_edge_1 = vec![0];
    let cit_edge_2 = vec![1];
    let cit_edge_3 = vec![100];
    println!("{:?}", Solution::h_index(cit));
    println!("{:?}", Solution::h_index(cit1));
    println!("{:?}", Solution::h_index(cit2));
    println!("{:?}", Solution::h_index(cit_edge));
    println!("{:?}", Solution::h_index(cit_edge_1));
    println!("{:?}", Solution::h_index(cit_edge_2));
    println!("{:?}", Solution::h_index(cit_edge_3));
}
