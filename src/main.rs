use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let total_numbers = 10_000;
    let mut arr: Vec<i32> =  (0..total_numbers).map(|_| rng.gen()).collect();

    // println!("Arr:");
    // for val in &arr {
    //     println!("{}", val)
    // }
    
    println!("Starting sorting...");
    merge_sort(&mut arr);
    println!("Array sorted.");
    // println!("Arr after sort:");
    // for val in &arr {
    //     println!("{}", val)
    // }
}

fn merge_sort(arr: &mut Vec<i32>) {
    let slice = arr.as_mut_slice();
    merge_sort1(slice);
}

fn merge_sort1(arr: &mut [i32]) {
    let arr_len = arr.len();
    if arr_len == 1 {
        return;
    }

    let middle = arr_len / 2;

    let (low_part, high_part) = arr.split_at_mut(middle);

    merge_sort1(low_part);
    merge_sort1(high_part);

    let sorted_arr = merge(arr_len, low_part, high_part);

    arr[..arr_len].copy_from_slice(sorted_arr.as_slice());
}

fn merge(master_slice_len: usize, lower_slice: &[i32], higher_slice: &[i32]) -> Vec<i32> {
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

    let mut res_arr: Vec<i32> = Vec::with_capacity(master_slice_len);
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
