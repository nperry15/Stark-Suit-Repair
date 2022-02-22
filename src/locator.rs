use std::cmp::Ordering;
use std::collections::HashMap;

pub trait PriorityQueue<T: PartialOrd> {
    fn enqueue(&mut self, ele: T) -> ();
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
}

/**
    An optional definition of a Node struct you may find useful
**/
struct Node<T> {
    priority: i32,
    data: T,
}

/** 
    These traits are implemented for Nodes to make them comparable 
**/
impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.priority == other.priority
    }
}


/** 
    You must implement the above trait for the vector type 
**/
impl<T: PartialOrd> PriorityQueue<T> for Vec<T> {
    /**
        This functions pushes a given element onto the queue and
        reorders the queue such that the min heap property holds.
        See the project specifications for more details on how this
        works.
    **/
    fn enqueue(&mut self, ele: T) -> () {
        self.push(ele);
        let mut i = self.len() - 1;

        while i != 0 && self.get((i-1)/2) > self.get(i){
          self.swap(i, (i-1)/2);
          i = (i-1)/2;
        }
    }

    /**
        This function removes the root element from the queue and
        reorders the queue such that it maintains the min heap
        property.  See the project specifications for more details.
        You should return the deleted element in the form of an option.
        Return None if the queue was initially empty, Some(T) otherwise.
    **/
    fn dequeue(&mut self) -> Option<T> {
        if self.len() == 0{
          return None;
        }
        
        let ret = self.remove(0);
        if self.len() != 1{
          //fix heap structure
          let mut i = 0;
          loop{
            let mut smallest = i;
            let left = 2*i + 1;
            let right = 2*i + 2;
  
            if left < self.len() && self.get(left) < self.get(i){
              smallest = left;
            }
            if right < self.len() && self.get(right) < self.get(i){
              smallest = right;
            }
  
            if smallest == i{
              break;
            }
  
            self.swap(i, smallest);
            i = smallest;
          }
        }
  
        return Some(ret);
      }

    /**
        This function returns the element that would be removed
        if dequeue were called on the queue.  There should be no
        mutations to the queue.  Return the element in the form
        of an option.  Return None if the queue is empty, Some(T)
        otherwise.
    **/
    fn peek(&self) -> Option<&T> {
        if self.len() == 0{
          return None;
        }
        let reference = self.get(0).unwrap();
        return Some(&reference);
    }
}


/**
    You must implement this function that computes the orthogonal
    distance between two coordinates.  Remember, orthogonal distance
    is not like Euclidean distance.  See the specifications for more
    details.
**/
pub fn distance(p1: (i32,i32), p2: (i32,i32)) -> i32 {
    let (a, b) = p1;
    let (c, d) = p2;
    let mut total = 0;

    if a > c{
        total += a - c;
    }else{
        total += c - a;
    }

    if b > d{
        total += b - d;
    }else{
        total += d - b;
    }
    return total;
}

/**
    You must implement this function that determines which enemy Stark
    should battle and their coordinates.  You are given two hashmaps for
    allies and enemies.  Each maps a name to their current coordinates.
    You can assume that the allies hashmap will always have a name
    called "Stark" included.  Return the name and coordinates of the enemy
    Stark will battle in the form of a 3-tuple.  See the specifications
    for more details on how to choose which enemy.
**/
pub fn target_locator<'a>(allies: &'a HashMap<&String, (i32,i32)>, enemies: &'a HashMap<&String, (i32,i32)>) -> (&'a str,i32,i32) {
    let mut names_used = Vec::new();
    let mut best_distances = Vec::new();
    for (ally, a_dist) in allies.iter(){
      for (enemy, e_dist) in enemies.iter() {
        let n = Node{
          priority: distance(*a_dist, *e_dist),
          data: (*ally, *enemy)
        };
        best_distances.enqueue(n);
      }
    }

    let ene;

    for i in best_distances{
      let (a, b) = i.data;
      if !names_used.contains(a) && !names_used.contains(b){
        names_used.push(a.to_string());
        names_used.push(b.to_string());

        if a == "Stark"{
          ene = b;
          let (d1, d2) = enemies.get(ene).unwrap();
          return (ene, *d1, *d2);
        }
      }
    }

    return ("Thanos", 1, 1);
}


