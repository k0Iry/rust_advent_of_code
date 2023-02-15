pub mod day6 {
    use std::{
        collections::HashMap,
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    };

    #[derive(Clone, Copy)]
    pub enum MarkerType {
        Packet = 4,
        Message = 14,
    }

    impl crate::AdventOfCode {
        pub fn day6_detect_packet_marker(marker_type: MarkerType) -> i32 {
            let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            file_path.push("inputs/day6-input.txt");
            let file = BufReader::new(File::open(file_path).unwrap());

            let line = file.lines().next().unwrap().unwrap();
            let mut left: usize = 0;
            let mut right: usize = 0;
            let mut s: HashMap<u8, usize> = HashMap::new();
            while right - left < marker_type as usize {
                let c = line.as_bytes().get(right).unwrap();
                if s.contains_key(c) {
                    let oldleft = left;
                    left = s.get(c).unwrap() + 1;
                    for i in oldleft..left {
                        s.remove(line.as_bytes().get(i).unwrap());
                    }
                }
                s.insert(*c, right);
                right += 1;
            }
            right as i32
        }
    }
}
