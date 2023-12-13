struct Solution {}
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let char_array: Vec<char> = needle.chars().collect();
        let win_size = char_array.len();

        let mut haystack_array: Vec<char> = haystack.chars().collect();
        //Window over the haystack_array
    }
}
fn main() {
    println!("Hello, world!");
}
