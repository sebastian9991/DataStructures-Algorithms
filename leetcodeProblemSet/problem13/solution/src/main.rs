struct Solution {}
impl Solution {
    // Symbol       Value
    // I             1
    // V             5
    // X             10
    // L             50
    // C             100
    // D             500
    // M             1000
    pub fn roman_to_int(s: String) -> i32 {
        let v: Vec<char> = s.chars().collect();
        let mut sum = 0;
        let mut use_next_char = true;
        for i in 0..v.len() {
            if i != v.len() - 1 {
                if v[i] == 'I' {
                    if v[i + 1] == 'V' {
                        sum += 4;
                        use_next_char = false;
                        continue;
                    } else if v[i + 1] == 'X' {
                        sum += 9;
                        use_next_char = false;
                        continue;
                    }
                }

                if v[i] == 'X' {
                    if v[i + 1] == 'L' {
                        sum += 40;
                        use_next_char = false;
                        continue;
                    } else if v[i + 1] == 'C' {
                        sum += 90;
                        use_next_char = false;
                        continue;
                    }
                }

                if v[i] == 'C' {
                    if v[i + 1] == 'D' {
                        sum += 400;
                        use_next_char = false;
                        continue;
                    } else if v[i + 1] == 'M' {
                        sum += 900;
                        use_next_char = false;
                        continue;
                    }
                }
            }

            if use_next_char {
                match v[i] {
                    'I' => {
                        sum += 1;
                    }
                    'V' => {
                        sum += 5;
                    }
                    'X' => {
                        sum += 10;
                    }
                    'L' => {
                        sum += 50;
                    }
                    'C' => {
                        sum += 100;
                    }

                    'D' => {
                        sum += 500;
                    }
                    'M' => {
                        sum += 1000;
                    }
                    _ => {
                        return -1;
                    }
                }
            }
            use_next_char = true;
        }
        return sum;
    }
}
fn main() {
    let s: String = "III".to_string();
    println!("{}", Solution::roman_to_int(s));
    let s2: String = "LVIII".to_string();
    println!("{}", Solution::roman_to_int(s2));
    let s3: String = "MCMXCIV".to_string();
    println!("{}", Solution::roman_to_int(s3));
}
