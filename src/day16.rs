use std::{collections::VecDeque, fmt::Debug};

struct BitStream {
    data: VecDeque<bool>,
}

impl BitStream {
    fn new(input: &str) -> Self {
        let mut data = VecDeque::new();
        for c in input.chars() {
            let v = if c >= '0' && c <= '9' {
                c as usize - '0' as usize
            } else if c >= 'A' && c <= 'F' {
                10 + c as usize - 'A' as usize
            } else {
                continue;
            };
            data.push_back(((v >> 3) & 1) != 0);
            data.push_back(((v >> 2) & 1) != 0);
            data.push_back(((v >> 1) & 1) != 0);
            data.push_back(((v >> 0) & 1) != 0);
        }

        Self { data }
    }

    fn get(&mut self) -> Result<bool, ()> {
        if let Some(bit) = self.data.pop_front() {
            Ok(bit)
        } else {
            Err(())
        }
    }

    fn get_num(&mut self, bits: usize) -> Result<usize, ()> {
        let mut ret = 0;
        for i in 0..bits {
            let bit = self.get()?;
            ret *= 2;
            ret += if bit { 1 } else { 0 }
        }
        Ok(ret)
    }

    fn count(&self) -> usize {
        self.data.len()
    }
}

impl Debug for BitStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.data {
            f.write_str(if *c { "1" } else { "0" })?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum PacketData {
    Literal(usize),
    Operator(Vec<Packet>),
}

#[derive(Debug, Clone)]
struct Packet {
    version: u8,
    type_id: u8,
    data: PacketData,
}

impl Packet {
    fn new(stream: &mut BitStream) -> Result<Packet, ()> {
        let version = stream.get_num(3)? as u8;
        let type_id = stream.get_num(3)? as u8;

        let data = match type_id {
            4 => {
                let mut literal = 0;
                loop {
                    let block = stream.get_num(5)?;
                    literal *= 16;
                    literal += block & 0xF;
                    if block & 0x10 == 0 {
                        break;
                    }
                }
                PacketData::Literal(literal)
            }
            _ => {
                let mut sub = Vec::new();
                let length_type = stream.get()?;
                if length_type {
                    let count = stream.get_num(11)?;
                    for _ in 0..count {
                        sub.push(Packet::new(stream)?);
                    }
                } else {
                    let bits = stream.get_num(15)?;
                    let left = stream.count();
                    assert!(bits <= left);
                    while stream.count() + bits > left {
                        sub.push(Packet::new(stream)?);
                    }
                    assert!(stream.count() + bits == left);
                }
                PacketData::Operator(sub)
            }
        };

        Ok(Packet {
            version,
            type_id,
            data,
        })
    }

    fn version_sum(&self) -> usize {
        let mut ret = self.version as usize;
        if let PacketData::Operator(subs) = &self.data {
            for sub in subs {
                ret += sub.version_sum();
            }
        }
        ret
    }

    fn compute(&self) -> usize {
        match &self.data {
            PacketData::Literal(num) => *num,
            PacketData::Operator(subs) => {
                let values: Vec<usize> = subs.iter().map(|sub| sub.compute()).collect();

                return match self.type_id {
                    0 => values.iter().sum(),
                    1 => values.iter().product(),
                    2 => *values.iter().min().unwrap(),
                    3 => *values.iter().max().unwrap(),
                    5 => {
                        assert!(values.len() == 2);
                        if values[0] > values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        assert!(values.len() == 2);
                        if values[0] < values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        assert!(values.len() == 2);
                        if values[0] == values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    _ => {
                        panic!("invalid type_id");
                    }
                };
            }
        }
    }
}

pub fn solve() {
    let mut stream = BitStream::new(include_str!("inputs/16.txt"));
    let packet = Packet::new(&mut stream).expect("invalid data");

    println!("[day 16] task 1 = {}", packet.version_sum());
    println!("[day 16] task 2 = {}", packet.compute());
}
