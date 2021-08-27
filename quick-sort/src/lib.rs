use wasm_bindgen::prelude::*;
use rayon::join;

pub use wasm_bindgen_rayon::init_thread_pool;

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn quick_sort_timed(unsorted_array: Vec<i32>) -> Vec<i32> {
    let window = web_sys::window().expect("should have a window in this context");
    let performance = window
        .performance()
        .expect("performance should be available");
    let start = performance.now();

    let sorted_array = quick_sort(unsorted_array);

    let end = performance.now();

    console_log!("Single thread time to sort: {}", end - start);

    return sorted_array;
}

fn quick_sort(mut unsorted_array: Vec<i32>) -> Vec<i32> {
   // base case
    if unsorted_array.len() < 2 {
        return unsorted_array;
    }

    // choose pivot
    let pivot = unsorted_array.pop().unwrap();

    // create left and right arrays, then populate them
    let mut left = Vec::new();
    let mut right = Vec::new();

    unsorted_array.iter().for_each(|element| {
        // console_log!("element: {}, pivot: {}", *element, pivot);
        if *element < pivot {
            left.push(*element);
        } else {
            right.push(*element);
        }
    });

    let mut sorted_array = quick_sort(left);
    let mut pivot_vec = vec![pivot];
    let mut right_sorted = quick_sort(right);
    // quick sort left and right, then combine
    sorted_array.append(&mut pivot_vec);
    sorted_array.append(&mut right_sorted);

    return sorted_array; 
}

#[wasm_bindgen]
pub fn quick_sort_parallel_timed(unsorted_array: Vec<i32>) -> Vec<i32> {
    let window = web_sys::window().expect("should have a window in this context");
    let performance = window
        .performance()
        .expect("performance should be available");
    let start = performance.now();

    let sorted_array = quick_sort_parallel(unsorted_array);

    let end = performance.now();

    console_log!("Multiple threads time to sort: {}", end - start);

    return sorted_array;}

fn quick_sort_parallel(mut unsorted_array: Vec<i32>) -> Vec<i32> {
    // base case
    if unsorted_array.len() < 2 {
        return unsorted_array;
    }

    // choose pivot
    let pivot = unsorted_array.pop().unwrap();

    // create left and right arrays, then populate them
    let mut left = Vec::new();
    let mut right = Vec::new();

    unsorted_array.iter().for_each(|element| {
        // console_log!("element: {}, pivot: {}", *element, pivot);
        if *element < pivot {
            left.push(*element);
        } else {
            right.push(*element);
        }
    });

    let (mut sorted_array, mut right_sorted) = join(|| quick_sort_parallel(left),
                                                    || quick_sort_parallel(right));

    let mut pivot_vec = vec![pivot];
    // quick sort left and right, then combine
    sorted_array.append(&mut pivot_vec);
    sorted_array.append(&mut right_sorted);

    return sorted_array;
}

#[wasm_bindgen]
pub fn parallel_quick_sort_v2_timed(v: &mut [i32]) {
    let window = web_sys::window().expect("should have a window in this context");
    let performance = window
        .performance()
        .expect("performance should be available");
    let start = performance.now();

    parallel_quick_sort_v2(v);

    let end = performance.now();

    console_log!("Multiple threads (v2) time to sort: {}", end - start);
}


#[wasm_bindgen]
pub fn quick_sort_v2_timed(v: &mut [i32]) {
    let window = web_sys::window().expect("should have a window in this context");
    let performance = window
        .performance()
        .expect("performance should be available");
    let start = performance.now();

    quick_sort_v2(v);

    let end = performance.now();

    console_log!("Single thread (v2) time to sort: {}", end - start);
}

pub fn quick_sort_v2(v: &mut [i32]) {
    if v.len() > 1 {
        let mid = partition(v);
        let (lo, hi) = v.split_at_mut(mid);
        quick_sort_v2(lo);
        quick_sort_v2(hi);
    }
}

// From rust docs https://docs.rs/rayon/1.0.3/rayon/fn.join.html
pub fn parallel_quick_sort_v2(v: &mut [i32]) {
    if v.len() > 1 {
        let mid = partition(v);
        let (lo, hi) = v.split_at_mut(mid);
        rayon::join(|| parallel_quick_sort_v2(lo),
                    || parallel_quick_sort_v2(hi));
    }
 }

// Partition rearranges all items `<=` to the pivot
// item (arbitrary selected to be the last item in the slice)
// to the first half of the slice. It then returns the
// "dividing point" where the pivot is placed.
fn partition(v: &mut [i32]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}