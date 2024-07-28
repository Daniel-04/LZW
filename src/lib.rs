use std::collections::HashMap;

pub fn encode(s: &[u8]) -> Vec<u64> {
    let mut dict: HashMap<&[u8], u64> = HashMap::new();
    let nums: Vec<u8> = (0..=255).collect();
    for num in 0..=255 {
        dict.insert(&nums[num..=num], num as u64);
    }
    let mut codes = 256;

    let mut out: Vec<u64> = Vec::new();
    let mut begin = 0;
    let mut end = 0;
    while end < s.len() {
        if dict.contains_key(&s[begin..end+1]) {
            end += 1;
        }
        else {
            if let Some(&code) = dict.get(&s[begin..end]) {
                out.push(code);
            }
            dict.insert(&s[begin..end+1], codes);
            codes += 1;
            end += 1;
            begin = end-1;
        }
    }
    if let Some(&code) = dict.get(&s[begin..end]) {
        out.push(code);
    }

    out
}

pub fn decode(c: &[u64]) -> Vec<u8> {
    let mut dict: HashMap<u64, Vec<u8>> = (0..=255).map(|b| (b as u64, vec![b])).collect();
    let mut codes = 256;

    let mut out: Vec<u8> = Vec::new();
    if let Some(previous) = c.first() {
        if let Some(mut previous_entry) = dict.get(previous).cloned() {
            out.extend(&previous_entry);
            for code in &c[1..] {
                if let Some(entry) = dict.get(code) {
                    let tmp = entry.clone();
                    out.extend(entry);
                    let &ch = entry.first().unwrap();
                    previous_entry.push(ch);
                    dict.insert(codes, previous_entry);
                    codes += 1;
                    previous_entry = tmp;
                }
                else {
                    let mut entry = previous_entry;
                    entry.push(*entry.first().unwrap());
                    previous_entry = entry.clone();
                    out.extend(&entry);
                    dict.insert(codes, entry);
                    codes += 1;
                }
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e_sequence() {
        let nums = [1, 2, 3];
        let result = encode(&nums);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn e_repeats() {
        let nums = [1, 2, 1, 2, 1, 2, 1, 2];
        let result = encode(&nums);
        assert_eq!(result, vec![1, 2, 256, 258, 2]);
    }

    #[test]
    fn e_repeats_and_change() {
        let nums = [1, 2, 1, 2, 1, 3, 1, 2];
        let result = encode(&nums);
        assert_eq!(result, vec![1, 2, 256, 1, 3, 256]);
    }

    #[test]
    fn d_sequence() {
        let nums = [1, 2, 3];
        let result = decode(&nums);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn d_pair() {
        let nums = [1, 2, 256];
        let result = decode(&nums);
        assert_eq!(result, vec![1, 2, 1, 2]);
    }

    #[test]
    fn d_palindrome() {
        let nums = [1, 2, 256, 258];
        let result = decode(&nums);
        assert_eq!(result, vec![1, 2, 1, 2, 1, 2, 1]);
    }
}
