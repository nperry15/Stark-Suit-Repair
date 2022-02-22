/**
    Returns the sum 1 + 2 + ... + n
    If n is less than 0, return -1
**/
pub fn gauss(n: i32) -> i32 {
    if n < 0 {
        return -1;
    }
    return gauss_helper(n);
}

pub fn gauss_helper(n: i32) -> i32 {
    if n < 0 {
        return 0;
    }
    return n + gauss_helper(n - 1);
}

/**
    Returns the number of elements in the list that 
    are in the range [s,e]
**/
pub fn in_range(ls: &[i32], s: i32, e: i32) -> i32 {
    let mut count = 0;
    for i in ls{
      if i >= &s && i <= &e {
        count += 1;
      }
    }
    return count;
}

/**
    Returns true if target is a subset of set, false otherwise

    Ex: [1,3,2] is a subset of [1,2,3,4,5]
**/
pub fn subset<T: PartialEq>(set: &[T], target: &[T]) -> bool {
    let mut is_subset = true;
    for i in target{
      if !set.contains(i) {
        is_subset = false;
      }
    }
    return is_subset;
}

/**
    Returns the mean of elements in ls. If the list is empty, return None
    It might be helpful to use the fold method of the Iterator trait
**/
pub fn mean(ls: &[f64]) -> Option<f64> {
    if ls.len() == 0{
      return None;
    }
    let mut mean = 0.0;
    for i in ls{
        mean += i;
    }
    mean = mean / ls.len() as f64;
    return Some(mean);
}
/**
    Converts a binary number to decimal, where each bit is stored in order in the array
    
    Ex: to_decimal of [1,0,1,0] returns 10
**/
pub fn to_decimal(ls: &[i32]) -> i32 {
    let mut ret = 0;
    let mut count = 1;
    for i in ls.iter().rev() {
          match i {
              &1 => {ret += count;
                    count *= 2;}
              _ => {count *= 2;}
          }
      }
  
    return ret;
  }

/**
    Decomposes an integer into its prime factors and returns them in a vector
    You can assume factorize will never be passed anything less than 2

    Ex: factorize of 36 should return [2,2,3,3] since 36 = 2 * 2 * 3 * 3
**/
pub fn factorize(n: u32) -> Vec<u32> {
    let mut ret = Vec::new();
    let mut x = n;
    while x % 2 == 0{
      ret.push(2);
      x /= 2;
    }

    for i in (3..((x as f64).sqrt() as u32)).step_by(2){
      while x % i == 0{
        ret.push(i);
        x /= i;
      }
    }

    if x > 2{
      ret.push(n);
    }

    return ret;
}

/** 
    Takes all of the elements of the given slice and creates a new vector.
    The new vector takes all the elements of the original and rotates them, 
    so the first becomes the last, the second becomes first, and so on.
    
    EX: rotate [1,2,3,4] returns [2,3,4,1]
**/
pub fn rotate(lst: &[i32]) -> Vec<i32> {
let mut ret = Vec::new();
    if lst.len() < 2{
        for i in lst{
        ret.push(*i);
        }
        return ret;
    }

    let mut i = 1;
    while i < lst.len(){
        ret.push(lst[i]);
        i += 1;
    }

    ret.push(lst[0]);

    return ret;
}  

/**
    Returns true if target is a subtring of s, false otherwise
    You should not use the contains function of the string library in your implementation
    
    Ex: "ace" is a substring of "rustacean"
**/
pub fn substr(s: &String, target: &str) -> bool {
    if target.len() > s.len(){
      return false;
    }
    if target.len() == 0 && s.len() == 0{
      return true;
    }
    if s.len() == 0{
      return true;
    }

    let target_string = target.to_string();
    let mut target_vec: Vec<_> = target_string.split("").collect();
    let mut s_vec: Vec<_> =  s.split("").collect();
    target_vec.pop();
    s_vec.pop();
    target_vec.remove(0);
    s_vec.remove(0);

    let mut i = 0;
    while i < s.len(){
      if s_vec[i] == target_vec[0]{
        let mut x = i;
        let mut n = 0;
        let mut break_early = false;
        while x < target.len() && !break_early{
          if s_vec[x] != target_vec[n]{
            break_early = true;
          }
          n += 1;
          x += 1;
        }

        if !break_early{
          return true;
        }
          
      }
      i += 1;
    }

    return false;
}

/**
    Takes a string and returns the first longest substring of consecutive equal characters

    EX: longest_sequence of "ababbba" is Some("bbb")
    EX: longest_sequence of "aaabbb" is Some("aaa")
    EX: longest_sequence of "xyz" is Some("x")
    EX: longest_sequence of "" is None
**/
pub fn longest_sequence(s: &str) -> Option<&str> {
    if s.len() < 1{
      return None;
    }
    
    let mut s_vec: Vec<_> =  s.split("").collect();
    s_vec.pop();
    s_vec.remove(0);

    let mut longest_run = 0;
    let mut longest_index = 0;
    let mut curr_run = 0;
    let mut i = 0;
    while i < s_vec.len() - 1{
      if s_vec[i] == s_vec[i+1]{
        curr_run += 1;
      }else{
        if curr_run > longest_run{
          longest_run = curr_run;
          curr_run = 0;
          longest_index = i - longest_run;
        }
      }
      i += 1;
    }
    return Some(&s[longest_index..(longest_index + longest_run + 1)]);
}