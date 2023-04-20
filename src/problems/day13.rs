pub mod day13 {
    use std::{
        cmp::Ordering,
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    };

    #[derive(Debug, Clone)]
    enum PacketData {
        Integer(u32),
        List(Vec<PacketData>),
    }

    impl PacketData {
        fn new(packets: &str, start: usize) -> (Self, usize) {
            let mut packet_data: Vec<PacketData> = vec![];
            let mut new_start = start;
            for (i, c) in packets.chars().skip(start).enumerate() {
                if i + start <= new_start {
                    continue;
                }
                if c == '[' {
                    let (new_data, end) = Self::new(packets, i + start);
                    new_start = end;
                    packet_data.push(new_data);
                } else if c.is_ascii_alphanumeric() {
                    if c == 'T' {
                        packet_data.push(Self::Integer(10))
                    } else {
                        packet_data.push(Self::Integer(c.to_digit(10).unwrap()))
                    }
                } else if c == ']' {
                    return (Self::List(packet_data), i + start);
                }
            }
            (Self::List(packet_data), packets.len())
        }

        fn compare(&self, other: &PacketData) -> Ordering {
            match (self, other) {
                (PacketData::List(left), PacketData::List(right)) => {
                    for (p1, p2) in left.iter().zip(right.iter()) {
                        let result = p1.compare(p2);
                        if result != Ordering::Equal {
                            return result;
                        }
                    }
                    left.len().cmp(&right.len())
                }
                (PacketData::Integer(left), PacketData::Integer(right)) => left.cmp(right),
                (PacketData::List(_), PacketData::Integer(_)) => {
                    self.compare(&PacketData::List(vec![other.clone()]))
                }
                (PacketData::Integer(_), PacketData::List(_)) => {
                    PacketData::List(vec![self.clone()]).compare(other)
                }
            }
        }
    }

    impl crate::AdventOfCode {
        pub fn day13_right_order() -> i32 {
            let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            file_path.push("inputs/day13-input.txt");
            let file = BufReader::new(File::open(file_path).unwrap());
            let mut iter = file.lines();

            let mut result = 0;

            let mut index = 0;
            while let (Some(Ok(p1)), Some(Ok(p2))) = (iter.next(), iter.next()) {
                index += 1;
                let left = p1.replace("10", "T");
                let right = p2.replace("10", "T");

                if PacketData::new(&left, 0)
                    .0
                    .compare(&PacketData::new(&right, 0).0)
                    == Ordering::Less
                {
                    result += index;
                }

                iter.next();
            }
            result
        }
    }
}
