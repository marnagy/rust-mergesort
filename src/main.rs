use rand::prelude::*;
use std::cmp::Ord;
use std::marker::Copy;
use std::thread;
use std::sync::{Arc, Mutex};
use std::rc::Rc;

fn main() {
    let mut rng = rand::thread_rng();
    let total_numbers = 5;
    let threads = 1;

    if threads < 1 {
        panic!("Cannot run merge sort on {0} threads.", threads);
    }

    let mut arr: Vec<i8> = (0..total_numbers).map(|_| rng.gen()).collect();

    println!("Arr:");
    for val in &arr {
        println!("{}", val)
    }

    println!("Starting sorting...");
    merge_sort(&mut arr, threads);
    println!("Array sorted.");

    println!("Arr after sort:");
    for val in &arr {
        println!("{}", val)
    }
}

fn merge_sort<T: Ord + Copy + std::marker::Send>(arr: &mut Vec<T>, threads: i8) {
    let slice = arr.as_mut_slice();
    merge_sort1(slice, threads);
}

fn merge_sort1<T: Ord + Copy + std::marker::Send>(arr: &mut [T], threads: i8) {
    let arr_len = arr.len();

    if arr_len == 1 {
        return;
    }

    let sorted_arr: Vec<T>;
    {
        let middle = arr_len / 2;
        let (low_part, high_part) = arr.split_at_mut(middle);
        let low_part_arc = Arc::from( Mutex::new(low_part) );
        let high_part_arc = Arc::from( Mutex::new(high_part) );
        let low_threads = threads / 2;
        let high_threads = threads - low_threads;

        let threaded_low_arc = low_part_arc.clone();
        let threaded_high_arc = high_part_arc.clone();
        
        let handle_low = thread::spawn(move || {
            if let Ok(x) = threaded_low_arc.lock() {
                merge_sort1(*x, low_threads);
            }
        });

        // let handle_high = thread::spawn(move || {
        //     if let Ok(x) = threaded_high_arc.lock() {
        //         merge_sort1(*x, high_threads);
        //     }
        // });

        sorted_arr = merge(arr_len, low_part, high_part);
    }

    arr.copy_from_slice(sorted_arr.as_slice());
}

fn merge<T: Ord + Copy + std::marker::Send>(master_slice_len: usize, lower_slice: &[T], higher_slice: &[T]) -> Vec<T> {
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
