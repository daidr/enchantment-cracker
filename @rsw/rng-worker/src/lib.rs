mod simple_random;
mod utils;

extern crate js_sys;
use std::cmp;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// console.log
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Atomics.add rename to BigIntAdd
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Atomics, js_name = store,catch)]
    fn AtomicsBigIntStore(typed_array: &JsValue, index: u32, value: i64) -> Result<i64, JsValue>;
}

#[wasm_bindgen]
pub fn first_input(
    booksheleves: i32,
    slot1: i32,
    slot2: i32,
    slot3: i32,
    core_count: i32,
    seed_buf: js_sys::SharedArrayBuffer,
    seed_searched_buf: js_sys::SharedArrayBuffer,
    abort_requested_buf: js_sys::SharedArrayBuffer,
) -> u64 {
    //utils::set_panic_hook();
    let block_size: i32 = 2147483647 / 20 / core_count - 1;
    let seed: js_sys::Int32Array = js_sys::Int32Array::new(&seed_buf);
    let seed_searched: js_sys::BigInt64Array = js_sys::BigInt64Array::new(&seed_searched_buf);
    let abort_requested: js_sys::Int8Array = js_sys::Int8Array::new(&abort_requested_buf);

    let two_shelves: i32 = booksheleves * 2;
    let half_shelves: i32 = booksheleves / 2 + 1;
    let shelves_plus_one: i32 = booksheleves + 1;

    let first_early: i32 = slot1 * 3 + 2;
    let second_early: i32 = slot2 * 3 / 2;
    let second_sub_one: i32 = slot2 - 1;

    let mut my_list: [i32; 10000] = [0; 10000];
    let mut pos: u32 = 0;
    let mut my_rng = simple_random::SimpleRandom::new();
    let mut count: u64 = 0;

    loop {
        if js_sys::Atomics::load(&abort_requested, 0).unwrap() == 1 {
            log("abort requested");
            break;
        }

        let cur_seed = js_sys::Atomics::load(&seed, 0).unwrap();
        let last = cur_seed.wrapping_add(block_size);
        if last < cur_seed {
            break; // overflow
        }

        if js_sys::Atomics::compare_exchange(&seed, 0, cur_seed, last) == Ok(cur_seed) {
            for i in cur_seed..last {
                my_rng.set_seed(i.into());
                let ench1r1 = my_rng.next_int(8) + half_shelves;
                if ench1r1 > first_early {
                    continue;
                }

                let ench1 = (ench1r1 + my_rng.next_int(shelves_plus_one)) / 3;
                if ench1 < 1 {
                    if slot1 != 1 {
                        continue;
                    }
                }
                if ench1 != slot1 {
                    continue;
                }

                let ench2r1 = my_rng.next_int(8) + half_shelves;
                if ench2r1 > second_early {
                    continue;
                }

                let ench2 = (ench2r1 + my_rng.next_int(shelves_plus_one)) * 2 / 3;
                if ench2 != second_sub_one {
                    continue;
                }

                let ench3 = my_rng.next_int(8) + half_shelves + my_rng.next_int(shelves_plus_one);
                if cmp::max(ench3, two_shelves) != slot3 {
                    continue;
                }

                my_list[pos as usize] = i;
                pos += 1;

                if pos == 10000 {
                    AtomicsBigIntStore(&seed_searched, 0, i as i64 + 2147483648).unwrap();
                    count += 10000;
                    pos = 0;
                }
            }
        }
    }
    count += pos as u64;
    // AtomicsBigIntAdd(&seed_searched, 0, pos as i64).unwrap();
    return count;
}
