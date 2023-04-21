pub mod day13 {
    use std::{
        cmp::Ordering,
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    };

    #[derive(Clone, PartialEq, PartialOrd, Eq)]
    enum PacketData {
        Integer(u32),
        List(Vec<PacketData>),
    }

    // impl Display for PacketData {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         match self {
    //             PacketData::Integer(i) => write!(f, "{i}"),
    //             PacketData::List(v) => {
    //                 write!(f, "[")?;
    //                 for packet in v {
    //                     packet.fmt(f).unwrap();
    //                 }
    //                 write!(f, "]")?;
    //                 Ok(())
    //             }
    //         }
    //     }
    // }

    impl Ord for PacketData {
        fn cmp(&self, other: &Self) -> Ordering {
            match (self, other) {
                (PacketData::List(left), PacketData::List(right)) => {
                    for (p1, p2) in left.iter().zip(right.iter()) {
                        let result = p1.cmp(p2);
                        if result != Ordering::Equal {
                            return result;
                        }
                    }
                    left.len().cmp(&right.len())
                }
                (PacketData::Integer(left), PacketData::Integer(right)) => left.cmp(right),
                (PacketData::List(_), PacketData::Integer(_)) => {
                    self.cmp(&PacketData::List(vec![other.clone()]))
                }
                (PacketData::Integer(_), PacketData::List(_)) => {
                    PacketData::List(vec![self.clone()]).cmp(other)
                }
            }
        }
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
    }

    impl crate::AdventOfCode {
        pub fn day13_right_order() -> (i32, i32) {
            let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            file_path.push("inputs/day13-input.txt");
            let file = BufReader::new(File::open(file_path).unwrap());
            let mut iter = file.lines();

            let mut result = 0;

            let mut index = 0;
            let mut packets = vec![];
            while let (Some(Ok(p1)), Some(Ok(p2))) = (iter.next(), iter.next()) {
                index += 1;
                let left = p1.replace("10", "T");
                let right = p2.replace("10", "T");

                let left = PacketData::new(&left, 0).0;
                let right = PacketData::new(&right, 0).0;

                if left.cmp(&right) == Ordering::Less {
                    result += index;
                }

                packets.push(left);
                packets.push(right);

                iter.next();
            }

            packets.sort_by(|a, b| a.cmp(b));

            let div1_packet =
                PacketData::List(vec![PacketData::List(vec![PacketData::Integer(2)])]);
            let div2_packet =
                PacketData::List(vec![PacketData::List(vec![PacketData::Integer(6)])]);

            let index1 = match packets.binary_search(&div1_packet) {
                Ok(_) => panic!("shouldn't exist"),
                Err(pos) => pos,
            } + 1;
            let index2 = match packets.binary_search(&div2_packet) {
                Ok(_) => panic!("shouldn't exist"),
                Err(pos) => pos,
            } + 2;

            (result, (index1 * index2) as i32)
        }
    }
}
