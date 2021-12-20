use aoc::prelude::*;

type Input = String;

fn parser(path: &Path) -> anyhow::Result<Input> {
    let reader = BufReader::new(File::open(path)?);
    let mut lines = reader.lines();

    let line = lines.next().unwrap()?;
    assert!(lines.next().is_none());
    Ok(line.trim_end().to_owned())
}

fn to_bits(s: &str) -> String {
    let mut out = String::new();
    for ch in s.chars() {
        let bits = match ch {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!(),
        };
        out.push_str(bits);
    }
    out
}

/*
struct Bitstream {
    buffer: VecDeque<u8>,
    free_bits: u8,
}

impl Bitstream {
    fn new(data: Vec<u8>) -> Bitstream {
        Bitstream {
            buffer: VecDeque::from(data),
            free_bits: 8,
        }
    }

    fn get_bits(&mut self, mut n: u8) -> u8 {
        assert!(n < 8);
        assert!(self.free_bits > 0);

        let mut output = 0;
        while 0 < n {
            let to_pull = if self.free_bits < n {
                self.free_bits
            } else {
                n
            };

            let mask = u8::try_from((1u16 << to_pull) - 1).unwrap();
            let downshift = self.free_bits - to_pull;
            let these_bits = (self.buffer[0] >> downshift) & mask;
            output = (output << to_pull) | these_bits;
            self.free_bits = self.free_bits.checked_sub(to_pull).unwrap();
            n = n.checked_sub(to_pull).unwrap();

            if self.free_bits == 0 {
                self.buffer.pop_front();
                if !self.buffer.is_empty() {
                    self.free_bits = 8;
                }
            }
        }

        n
    }
}
*/
fn pull_bits(n: usize, s: &str) -> (u16, &str) {
    assert!(n <= s.len());
    let (front, back) = s.split_at(n);
    let mut out = 0;
    for c in front.chars() {
        let bit = match c {
            '0' => 0,
            '1' => 1,
            _ => panic!(),
        };
        out = (out << 1) | bit;
    }

    (out, back)
}

#[derive(Debug)]
struct Packet {
    version: u16,
    type_id: u16,
    data: PacketData,
}

impl Packet {
    fn sum_version(&self) -> u64 {
        let subpacket_sum = match &self.data {
            PacketData::Literal(_) => 0,
            PacketData::Subpackets(packets) => {
                packets.iter().map(|p| p.sum_version()).sum()
            }
        };
        subpacket_sum + u64::from(self.version)
    }

    fn eval_packet(&self, depth: usize) -> i64 {
        use PacketData::*;

        let (o, val) = match (self.type_id, &self.data) {
            (4, Literal(n)) => ('"', i64::try_from(*n).unwrap()),
            (0, Subpackets(sp)) => {
                assert!(sp.len() > 0);
                let mut sum: i64 = 0;
                for v in sp.iter().map(|p| p.eval_packet(depth+1)) {
                    sum = sum.checked_add(v).unwrap();
                }
                ('+', sum)
                //sp.iter().map(|p| p.eval_packet()).sum::<i64>()
            }
            (1, Subpackets(sp)) => {
                assert!(sp.len() > 0);
                let mut product: i64 = 1;
                for v in sp.iter().map(|p| p.eval_packet(depth+1)) {
                    product = product.checked_mul(v).unwrap();
                }
                ('*', product)
                //sp.iter().map(|p| p.eval_packet()).product::<i64>()
            }
            (2, Subpackets(sp)) => ('v', sp.iter().map(|p| p.eval_packet(depth+1)).min().unwrap()),
            (3, Subpackets(sp)) => ('^', sp.iter().map(|p| p.eval_packet(depth+1)).max().unwrap()),
            (5, Subpackets(sp)) => {
                assert!(sp.len() == 2);
                let first = sp[0].eval_packet(depth+1);
                let second = sp[1].eval_packet(depth+1);
                let r = if first > second {
                    1
                } else { 0 };
                ('>', r)
            }
            (6, Subpackets(sp)) => {
                assert!(sp.len() == 2);
                let first = sp[0].eval_packet(depth+1);
                let second = sp[1].eval_packet(depth+1);
                let r = if first < second {
                    1
                } else { 0 };
                ('<', r)
            }
            (7, Subpackets(sp)) => {
                assert!(sp.len() == 2);
                let first = sp[0].eval_packet(depth+1);
                let second = sp[1].eval_packet(depth+1);
                let r = if first == second {
                    1
                } else { 0 };
                ('=', r)
            }
            _ => panic!(),
        };
        for _ in 0..depth {
            print!(" ");
        }
        println!("{} => val = {}", o, val);
        val
    }
}

#[derive(Debug)]
enum PacketData {
    Literal(u64),
    Subpackets(Vec<Packet>),
}

fn decode_packet(bitstream: &str) -> (Packet, &str) {
    let (version, rem) = pull_bits(3, bitstream);
    let (type_id, rem) = pull_bits(3, rem);

    match type_id {
        4 => {
            //println!("Parse literal packet.");
            let mut rem = rem;
            let mut value = 0;
            //println!("Literal");
            loop {
                let (bits, hrem) = pull_bits(5, rem);
                rem = hrem;
                value = (value << 4) | u64::from(bits & 0b1111);
                //println!("Got bits: {:0b}; value now {}", bits, value);
                if bits & 0b10000 == 0 {
                    break;
                }
            }
            let packet = Packet {
                version,
                type_id,
                data: PacketData::Literal(value),
            };
            (packet, rem)
        }
        _ => {
            let (length_type_id, rem) = pull_bits(1, rem);
            //println!("Parse subpacket container, version {}, type {}.", version, length_type_id);
            let mut subpackets = Vec::new();
            let final_rem = if length_type_id == 0 {
                let (total_subpacket_length, rem) = pull_bits(15, rem);
                //println!("Length in bits: {}", total_subpacket_length);
                let section_start = rem;
                let mut rem = rem;
                let mut bits_used = 0;
                while bits_used < total_subpacket_length {
                    //println!("loooop start, now at {:0x}", rem.as_ptr() as usize);
                    let (packet, hrem) = decode_packet(rem);
                    rem = hrem;
                    //println!("loooop, now at {:0x}", rem.as_ptr() as usize);
                    subpackets.push(packet);
                    let now_bits_used = (rem.as_ptr() as usize) - (section_start.as_ptr() as usize);
                    let now_bits_used = u16::try_from(now_bits_used).unwrap();
                    bits_used = now_bits_used;
                    //println!("{} bits used.", bits_used);
                }
                rem
            } else {
                let (number_of_packets, rem) = pull_bits(11, rem);
                let mut rem = rem;
                for _ in 0..number_of_packets {
                    let (packet, hrem) = decode_packet(rem);
                    rem = hrem;
                    subpackets.push(packet);
                }
                rem
            };
            let packet = Packet {
                version,
                type_id,
                data: PacketData::Subpackets(subpackets),
            };
            (packet, final_rem)
        }
    }
}

fn part_a(input: &Input) -> i64 {
    let bits = to_bits(&input);
    let (packet, _) = decode_packet(&bits);
    let version_sum = packet.sum_version();
    i64::try_from(version_sum).unwrap()
}

fn do_eval(s: &str) -> i64 {
    let bits = to_bits(&s);
    let (packet, _) = decode_packet(&bits);
    //println!("{:#?}", packet);
    packet.eval_packet(0)
}

fn test_eval(s: &str, expect: i64) {
    let bits = to_bits(&s);
    let (packet, _) = decode_packet(&bits);
    let answer = packet.eval_packet(0);
    if answer == expect {
        println!("{}: PASS", s);
    } else {
        println!("{}: FAIL", s);
        println!("{:#?}", packet);
    }
}

fn part_b(input: &Input) -> i64 {
    test_eval("C200B40A82", 3);
    test_eval("04005AC33890", 54);
    test_eval("880086C3E88112", 7);
    test_eval("CE00C43D881120", 9);
    test_eval("D8005AC2A8F0", 1);
    test_eval("F600BC2D8F", 0);
    test_eval("9C005AC2F8F0", 0);
    test_eval("9C0141080250320F1802104A08", 1);
    do_eval(input)
}

aoc::aoc!(parser, part_a, part_b, Some(16), Some(1));
