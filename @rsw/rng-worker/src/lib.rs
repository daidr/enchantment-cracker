mod simple_random;
mod utils;

extern crate js_sys;
use std::cmp;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

// create a vector to store possible seeds
static mut POSSIBLE_SEEDS: Vec<i32> = Vec::new();

#[wasm_bindgen]
pub fn first_input(
    booksheleves: i32,
    slot1: i32,
    slot2: i32,
    slot3: i32,
    core_count: i32,
    seed_buf: js_sys::SharedArrayBuffer,
    abort_requested_buf: js_sys::SharedArrayBuffer,
) -> i32 {
    //utils::set_panic_hook();
    let block_size: i32 = 2147483647 / 20 / core_count - 1;
    let seed: js_sys::Int32Array = js_sys::Int32Array::new(&seed_buf);
    let abort_requested: js_sys::Int8Array = js_sys::Int8Array::new(&abort_requested_buf);

    let two_shelves: i32 = booksheleves * 2;
    let half_shelves: i32 = booksheleves / 2 + 1;
    let shelves_plus_one: i32 = booksheleves + 1;

    let first_early: i32 = slot1 * 3 + 2;
    let second_early: i32 = slot2 * 3 / 2;
    let second_sub_one: i32 = slot2 - 1;

    let mut my_list: [i32; 100000] = [0; 100000];
    let mut pos: u32 = 0;
    let mut my_rng = simple_random::SimpleRandom::new();

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

                if pos == 100000 {
                    // store possible seeds
                    unsafe {
                        POSSIBLE_SEEDS.append(&mut my_list.to_vec());
                    }
                    pos = 0;
                }
            }
        }
    }
    // store possible seeds before pos
    unsafe {
        POSSIBLE_SEEDS.append(&mut my_list[0..pos as usize].to_vec());
        return POSSIBLE_SEEDS.len() as i32;
    }
}

fn get_generic_enchantability(rand: &mut simple_random::SimpleRandom, booksheleves: &i32) -> i32 {
    let first = rand.next_int(8);
    let second = rand.next_int(booksheleves + 1);
    return first + 1 + (booksheleves >> 1) + second;
}

fn get_levels_slot1(rand: &mut simple_random::SimpleRandom, booksheleves: &i32) -> i32 {
    let enchantability = get_generic_enchantability(rand, booksheleves) / 3;
    return cmp::max(enchantability, 1);
}

fn get_levels_slot2(rand: &mut simple_random::SimpleRandom, booksheleves: &i32) -> i32 {
    return get_generic_enchantability(rand, booksheleves) * 2 / 3 + 1;
}

fn get_levels_slot3(rand: &mut simple_random::SimpleRandom, booksheleves: &i32) -> i32 {
    let enchantability = get_generic_enchantability(rand, booksheleves);
    let twice_booksheleves = booksheleves * 2;
    return cmp::max(enchantability, twice_booksheleves);
}

#[wasm_bindgen]
pub fn get_last_few_seeds(
    start_seed: i32,
    booksheleves: i32,
    slot1: i32,
    slot2: i32,
    slot3: i32,
) -> js_sys::Array {
    let array = js_sys::Array::new();
    let mut my_rng = simple_random::SimpleRandom::new();
    for i in (start_seed - 1)..2147483647 {
        my_rng.set_seed((i + 1).into());
        if get_levels_slot1(&mut my_rng, &booksheleves) == slot1 {
            if get_levels_slot2(&mut my_rng, &booksheleves) == slot2 {
                if get_levels_slot3(&mut my_rng, &booksheleves) == slot3 {
                    array.push(&JsValue::from(i + 1));
                }
            }
        }
    }
    return array;
}
