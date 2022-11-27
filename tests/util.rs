use std::{collections::hash_map::DefaultHasher, fs, hash::BuildHasherDefault};

use all_of_hashtable::{HashMap, Stat};
use plotly::{
    common::Title,
    layout::{Axis, AxisType},
    Histogram, ImageFormat, Layout, Plot,
};
use rand::{
    prelude::{SliceRandom, ThreadRng},
    thread_rng, Rng,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Operation {
    Insert,
    Lookup,
    Remove,
}

#[derive(PartialEq)]
enum OperationType {
    Some, // the operation for existing key on the map
    None, // the operation for not existing key on the map
}

pub fn stress_hashmap<T>(map: &mut T, iter: u64, print: bool)
where
    T: HashMap<u64, u64, BuildHasherDefault<DefaultHasher>>,
{
    let gen_not_existing_key = |rng: &mut ThreadRng, map: &std::collections::HashMap<u64, u64>| {
        let mut key = rng.gen();

        for _ in 0..10 {
            if !map.contains_key(&key) {
                return Ok(key);
            }

            key = rng.gen();
        }

        Err(())
    };

    let ops = [Operation::Insert, Operation::Lookup, Operation::Remove];
    let types = [OperationType::Some, OperationType::None];

    let mut ref_map = std::collections::HashMap::new();
    let mut rng = thread_rng();

    for i in 1..=iter {
        let t = types.choose(&mut rng).unwrap();
        let ref_map_keys = ref_map.keys().collect::<Vec<&u64>>();
        let existing_key = ref_map_keys.choose(&mut rng);

        if existing_key.is_none() || *t == OperationType::None {
            // run operation with not existing key
            let not_existing_key = if let Ok(key) = gen_not_existing_key(&mut rng, &ref_map) {
                key
            } else {
                continue;
            };

            match ops.choose(&mut rng).unwrap() {
                Operation::Insert => {
                    // should success
                    let value: u64 = rng.gen();

                    if print {
                        println!(
                            "[{:0>10}] InsertNone: ({:?}, {})",
                            i, not_existing_key, value
                        );
                    }
                    assert_eq!(ref_map.insert(not_existing_key.clone(), value), None);
                    assert_eq!(map.insert(&not_existing_key, value), Ok(()));
                }
                Operation::Lookup => {
                    // should fail
                    if print {
                        println!("[{:0>10}] LookupNone: ({:?}, None)", i, not_existing_key);
                    }
                    assert_eq!(ref_map.get(&not_existing_key), None);
                    assert_eq!(map.lookup(&not_existing_key), None);
                }
                Operation::Remove => {
                    // should fail
                    if print {
                        println!("[{:0>10}] RemoveNone: ({:?}, Err)", i, not_existing_key);
                    }
                    assert_eq!(ref_map.remove(&not_existing_key), None);
                    assert_eq!(map.remove(&not_existing_key), Err(()));
                }
            }
        } else {
            // run operation with existing key
            let existing_key = (*existing_key.unwrap()).clone();

            match ops.choose(&mut rng).unwrap() {
                Operation::Insert => {
                    // should fail
                    let value: u64 = rng.gen();

                    if print {
                        println!("[{:0>10}] InsertSome: ({:?}, {})", i, existing_key, value);
                    }
                    assert_eq!(map.insert(&existing_key, value), Err(value));
                }
                Operation::Lookup => {
                    // should success
                    let value = ref_map.get(&existing_key);

                    if print {
                        println!(
                            "[{:0>10}] LookupSome: ({:?}, {})",
                            i,
                            existing_key,
                            value.unwrap()
                        );
                    }
                    assert_eq!(map.lookup(&existing_key), value);
                }
                Operation::Remove => {
                    // should success
                    let value = ref_map.remove(&existing_key);

                    if print {
                        println!(
                            "[{:0>10}] RemoveSome: ({:?}, {})",
                            i,
                            existing_key,
                            value.unwrap()
                        );
                    }
                    assert_eq!(map.remove(&existing_key).ok(), value);

                    // early stop code if the remove has any problems
                    // for key in ref_map.keys().collect::<Vec<&K>>() {
                    //     assert_eq!(map.lookup(key).is_some(), true, "the key {:?} is not found.", key);
                    // }
                }
            }
        }
    }
}

pub fn draw_stat(stat: Stat, file_name: &str) {
    let mut title: String;

    if file_name.contains("/") {
        let last_slash = file_name.rfind("/").unwrap();

        fs::create_dir_all(&file_name[0..last_slash]).unwrap();

        title = file_name[(last_slash + 1)..].to_string();
    } else {
        title = file_name.to_string();
    }

    if title.contains(".") {
        title = title[0..title.find(".").unwrap()].to_string();
    }

    let mut plot = Plot::new();
    plot.set_layout(
        Layout::new()
            .x_axis(Axis::new().dtick(1.))
            .y_axis(Axis::new().type_(AxisType::Log))
            .title(Title::new(&title)),
    );
    plot.add_trace(Histogram::new(stat.insert_psl).name("Insert PSL"));
    plot.add_trace(Histogram::new(stat.lookup_psl).name("Lookup PSL"));
    plot.add_trace(Histogram::new(stat.remove_psl).name("Remove PSL"));

    plot.write_image(file_name, ImageFormat::PNG, 1280, 720, 1.0);
}
