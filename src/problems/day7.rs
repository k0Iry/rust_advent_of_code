pub mod day7 {
    use std::{
        collections::HashMap,
        fs::File,
        io::{BufRead, BufReader},
        ops::Index,
        path::PathBuf,
    };

    const DISK_SIZE: u32 = 70000000;
    const REQUIRED_SIZE: u32 = 30000000;

    impl crate::AdventOfCode {
        pub fn day7_sum_of_folders_size() -> (u32, u32) {
            let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            file_path.push("inputs/day7-input.txt");
            let file = BufReader::new(File::open(file_path).unwrap());

            let mut path = String::new();
            let mut dir_sizes: HashMap<String, u32> = HashMap::new();
            let mut occupied_size = 0;

            for line in file.lines() {
                let line = line.unwrap();
                let splits: Vec<&str> = line.split(' ').collect();
                match *splits.index(0) {
                    "$" => match *splits.index(1) {
                        "cd" => match *splits.index(2) {
                            ".." => {
                                let sz = *dir_sizes.get(&path).unwrap();
                                path.drain(path.rfind('/').unwrap()..);
                                if let Some(current_sz) = dir_sizes.get_mut(&path) {
                                    *current_sz += sz;
                                } else {
                                    dir_sizes.insert(path.clone(), sz);
                                }
                            }
                            dir => {
                                path.push('/');
                                path.push_str(dir);
                            }
                        },
                        "ls" => continue,
                        other => panic!("Unrecognized command: {other}"),
                    },
                    file_or_dir => {
                        if let Ok(sz) = file_or_dir.parse::<u32>() {
                            occupied_size += sz;
                            if let Some(current_sz) = dir_sizes.get_mut(&path) {
                                *current_sz += sz;
                            } else {
                                dir_sizes.insert(path.clone(), sz);
                            }
                        }
                    }
                }
            }

            let reserve = REQUIRED_SIZE - (DISK_SIZE - occupied_size);
            let mut result = 0;
            let mut result2 = u32::MAX;
            for (_k, v) in dir_sizes {
                if v <= 100000 {
                    result += v;
                } else if v >= reserve {
                    result2 = std::cmp::min(result2, v);
                }
            }
            (result, result2)
        }
    }
}
