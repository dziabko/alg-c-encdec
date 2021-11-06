#![allow(unused)]
use std::str;
use std::fs;
fn main() {
    // Test1
    let passed_string = "ababcbababaa";
    let encoded_str = compBuff(passed_string);
    let dec_str = decompBuff(encoded_str);
    println!("Test1 Passed: {:?}", passed_string == dec_str);
}

pub fn compBuff(string: &str) -> Vec<(i32,usize, char)> {
    // Buffer of encoded strings, lookahead buffer
    let mut buffer = String::from("");
    let mut cur_str = String::from("");
    let b = buffer.clone();
    let mut prev_match_index: i32 = -1;
    let mut result = Vec::new();

    // Enumerate over every char in the string 
    for (i, c) in string.chars().enumerate() {
        // If on last char, just add last string sequence & last char
        if (i == (string.len() - 1)) {
            let len1 = cur_str.len();
            let offset = i as i32 - prev_match_index - len1 as i32;
            let tuple = (offset, len1, c);
            result.push(tuple);
            return result;
        }
        // Adding char to our lookahead buffer
        cur_str.push(c);
        
        // If not contained within buffer, add to buffer, and encode string
        // Find the previous location of our cur_str in the buffer
        let res = buffer.rfind(&cur_str);
        match res {
            None => {
                // If we haven't previously encoded the substring, our offset and length are 0
                if (prev_match_index == -1) {
                    // Append new unmatched string to buffer
                    buffer.push_str(&cur_str);

                    // Encoding (o, l, c)
                    let tuple = (0, 0, c);
                    result.push(tuple);

                    // Reset the curStr to encode next sequence of char
                    cur_str = String::from("");
                } else {
                    // Append new unmatched string to buffer
                    buffer.push_str(&cur_str);

                    // Length & offset of prev match
                    let len1 = cur_str.len() - 1;
                    let offset = i as i32 - prev_match_index - len1 as i32;

                    // Encoding (o, l, c)
                    let tuple = (offset, len1, c);
                    result.push(tuple);

                    // Reset the curStr to encode next sequence of char
                    cur_str = String::from("");
                }
            }
            Some(v) => {
                // Store prevMatch for when we find a non-matching string
                prev_match_index = v as i32;
            }
        }
    }
    return result;
}

pub fn decompBuff(compressed_str: Vec<(i32,usize, char)>) -> String {
    let mut res = String::from("");
    // Read through tuple, and reconstruct the string
    for tuple in compressed_str.iter() {
        let (offset, len, char) = tuple;
        if (*offset == 0) {
            res.push(*char);
        } else {
            // Read the char (len bytes) starting from offset
            let index1 = (res.len() as i32 - offset) as usize;
            let index2 = (index1 as i32 + *len as i32) as usize;
            let res2 = res.clone();
            let subset = &res2[index1..index2];
            
            // Append the subset to result
            res.push_str(subset);
            res.push(*char);
        }
    }
    return res.to_string();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        // Test1
        let passed_string = "ababcbababaa";
        let encoded_str = compBuff(passed_string);
        let dec_str = decompBuff(encoded_str);
        println!("Test1 Passed: {:?}", passed_string == dec_str);
    }

    #[test]
    fn test_diff_perm() {
        // Test2 (test all permutations of (a,b,c,d) concatenated delimited by special char)
        let passed_string = "abcdab@bc#cd$abc%bcd^abcd";
        let encoded_str = compBuff(passed_string);
        let dec_str = decompBuff(encoded_str);
        println!("Test3 Passed: {:?}", passed_string == dec_str);
    }

    #[test]
    fn test_large_file() {
        // Test3 (Effeciency test for a random 416KB file, and file I/O)
        // Can create random byte files of any size using 'openssl rand -out rand6.txt -base64 $(( 2**14 * 25 * 3/4 ))'
        let filename = "rand6.txt";
        let passed_string = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let encoded_str = compBuff(&passed_string);
        let dec_str = decompBuff(encoded_str);
        println!("Test3 Passed: {:?}", passed_string == dec_str);
    }


}


