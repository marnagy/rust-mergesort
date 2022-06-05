use std::cmp::Ord;
use std::marker::{Copy, Send};

use crossbeam;

pub fn merge_sort_multithread<T: Ord + Copy + Send>(arr: &mut Vec<T>, threads: i8) {
    let slice: &mut [T] = arr.as_mut_slice();
    if threads == 1 {
        merge_sort1_singlethread(slice);
    }
    else if threads >= 2 {
        merge_sort1(slice, threads);
    }
    else {
        panic!("Cannot sort with {0} threads.", threads);
    }
}

pub fn merge_sort<T: Ord + Copy>(arr: &mut Vec<T>){
    let slice: &mut [T] = arr.as_mut_slice();
    merge_sort1_singlethread(slice);
}

fn merge_sort1_singlethread<T: Ord + Copy>(arr: &mut [T]) {
    let arr_len = arr.len();

    if arr_len == 1 {
        return;
    }

    let sorted_arr: Vec<T>;
    {
        let middle = arr_len / 2;
        let (low_part, high_part) = arr.split_at_mut(middle);
        
        merge_sort1_singlethread(low_part);
        merge_sort1_singlethread(high_part);

        sorted_arr = merge(arr_len, low_part, high_part);
    }

    arr.copy_from_slice(sorted_arr.as_slice());
}

fn merge_sort1<T: Ord + Copy + Send>(arr: &mut [T], threads: i8) -> &mut [T] {
    let arr_len = arr.len();

    if threads == 1 {
        merge_sort1_singlethread(arr);
        return arr;
    }

    let low_threads = threads / 2;
    let high_threads = threads - low_threads;

    let middle = arr_len / 2;
    let low_part: &mut [T];
    let high_part: &mut [T];
    (low_part, high_part) = arr.split_at_mut(middle);

    let sorted_arr = crossbeam::scope(|scope| {
        let handle1 = scope.spawn(|_| merge_sort1(low_part, low_threads));
        let handle2 = scope.spawn(|_| merge_sort1(high_part, high_threads));
        let low_part = handle1.join().unwrap();
        let high_part = handle2.join().unwrap();
        scope.spawn(move |_| merge(arr_len, low_part, high_part) ).join().unwrap()
    }).unwrap();
    
    arr.copy_from_slice(sorted_arr.as_slice());
    arr
}

fn merge<T: Ord + Copy>(master_slice_len: usize, lower_slice: &[T], higher_slice: &[T]) -> Vec<T> {
    let lower_len = lower_slice.len();
    let higher_len = higher_slice.len();

    if master_slice_len != lower_len + higher_len {
        panic!(
            "Inconsistent slice sizes:\nmaster: {}\nlower: {}\nhigher: {}",
            master_slice_len,
            lower_slice.len(),
            higher_slice.len()
        )
    }

    let mut res_arr: Vec<T> = Vec::with_capacity(master_slice_len);
    let mut lower_index = 0_usize;
    let mut higher_index = 0_usize;

    while lower_index < lower_len && higher_index < higher_len {
        let number_from_lower = lower_slice[lower_index];
        let number_from_higher = higher_slice[higher_index];

        if number_from_lower <= number_from_higher {
            res_arr.push(number_from_lower);
            lower_index += 1;
        } else {
            res_arr.push(number_from_higher);
            higher_index += 1;
        }
    }

    if lower_index == lower_len {
        while higher_index < higher_len {
            res_arr.push(higher_slice[higher_index]);
            higher_index += 1;
        }
    } else {
        while lower_index < lower_len {
            res_arr.push(lower_slice[lower_index]);
            lower_index += 1;
        }
    }

    res_arr
}
