use rand::prelude::*;

mod mergesort;
use mergesort::{merge_sort_multithread};

fn main() {
    let mut rng = rand::thread_rng();
    let total_numbers = 10_000_000;
    let threads: i8 = 4;

    if threads < 1 {
        panic!("Cannot run merge sort on {0} threads.", threads);
    }

    let mut arr: Vec<i64> = (0..total_numbers).map(|_| rng.gen()).collect();

    println!("Arr:");
    for val in &arr {
        println!("{}", val)
    }

    println!("Starting sorting...");
    merge_sort_multithread(&mut arr, threads);
    println!("Array sorted.");

    println!("Arr after sort:");
    for val in &arr {
        println!("{}", val)
    }
}

