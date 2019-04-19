#![feature(test)]
extern crate test;

use std::cmp::Ordering;

extern crate rand;
use rand::seq::SliceRandom;
use rand::thread_rng;
//Lines 1 and 2 are for test featuries.
//Ordering is used for the merge function.
//rand uses SliceRandom and thread_rng to shuffle the initial vector.


fn main() {
    let mut sort_vec: Vec<i32> = (0..10000000).collect();
    sort_vec.shuffle(&mut thread_rng());
    sort_vec = merge_sort(sort_vec);
    println!("{:?}", sort_vec);
}

fn merge_sort(sort_vec: Vec<i32>) -> Vec<i32> {
    let mut vec_a: Vec<i32> = Vec::new();
    let mut vec_b: Vec<i32> = Vec::new();
    //Creates two buffer vectors.

    if sort_vec.len() > 2 {
        vec_a.extend(sort_vec.iter().take(sort_vec.len() / 2));
        vec_b.extend(sort_vec.iter().skip(sort_vec.len() / 2));
        vec_a = merge_sort(vec_a);
        vec_b = merge_sort(vec_b);
        merge(vec_a, vec_b)
        //Extends vec_a and vec_b with their respective half of vector,
        //then recursively merge sorts before merging the values.
    } else {
        if sort_vec.len() == 1 || sort_vec[0] < sort_vec[1] {
            sort_vec
        } else {
            vec![sort_vec[1], sort_vec[0]]
        }
        //Bottom level of recursion when vector len <= 2,
        //returns self if ordered, returns swapped values otherwise.
    }

}

fn merge(vec_1: Vec<i32>, vec_2: Vec<i32>) -> Vec<i32> {
    let mut vec_merged: Vec<i32> = Vec::new();
    let mut left_index = 0;
    let mut right_index = 0;
    //Creates output buffer and 2 indices for merging.

    for _ in 0..(vec_1.len() + vec_2.len()) {

        if left_index == vec_1.len() {
            vec_merged.extend(vec_2.iter().skip(right_index));
            return vec_merged;
            //If all of vector 1's values have been read,
            //push vector 2 values to output buffer and return.
        } else if right_index == vec_2.len() {
            vec_merged.extend(vec_1.iter().skip(left_index));
            return vec_merged;
            //If all of vector 2's values have been read,
            //push vector 1 values to output buffer and return.
        } else {
            match vec_1[left_index].cmp(&vec_2[right_index]) {
                Ordering::Less | Ordering::Equal => {
                    vec_merged.push(vec_1[left_index]);
                    left_index += 1;
                }
                Ordering::Greater => {
                    vec_merged.push(vec_2[right_index]);
                    right_index += 1;
                }

            }
            //Compare both values at their current index, 
            //push the lesser (or left, if equal) value to buffer,
            //increment the corresponding index.
        }

    }

    //This bit is a bit messy, need to find alternative to if/else chain.
    vec_merged
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_merge(b: &mut Bencher) {
        b.iter(|| main());
    }
}
