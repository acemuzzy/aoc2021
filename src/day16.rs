pub struct Day16;

struct Tlv {
    data: Vec<u8>,
    cursor: usize,
}

impl std::fmt::Display for Tlv {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let output: String = self.data.iter().map(|d| d.to_string()).collect();
        writeln!(f, "{}", output)?;

        for ii in 0..self.data.len() {
            write!(f, "{}", if self.cursor == ii { "^" } else { " " })?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum PacketData {
    Literal(u64),
    SubPackets(Vec<Packet>),
}

#[derive(Debug)]
enum ID {
    Literal,
    Operator(u8),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    id: ID,
    packet_data: PacketData,
}

impl Tlv {
    fn read(&mut self) -> u8 {
        let b = self.data[self.cursor];
        self.cursor += 1;
        b
    }

    fn read_bits(&mut self, count: usize) -> Vec<u8> {
        let bits = self.data[self.cursor..self.cursor + count].to_vec();
        self.cursor += count;
        bits
    }

    fn parse(&mut self) -> Packet {
        // Read first three bits (V)
        let v_bits = self.read_bits(3);
        let version = Self::val_from_bits(v_bits).try_into().unwrap();
        println!("V={:?}", version);

        // Read next three bits (T)
        let t_bits = &self.data[self.cursor..self.cursor + 3];
        let mut id = 0;
        println!("T={:?}", t_bits);
        let mut val = 4;
        for t in t_bits.iter().take(3) {
            if *t == 1 {
                id += val;
            }
            val /= 2;
            self.cursor += 1;
        }
        let id = match id {
            4 => ID::Literal,
            _ => ID::Operator(id),
        };

        // Read the next lot...
        let packet_data = match id {
            ID::Literal => {
                println!("Literal");
                PacketData::Literal(self.parse_literal())
            }
            _ => {
                let i_bit = self.read();
                let mut inner_packets: Vec<Packet> = vec![];

                match i_bit {
                    0 => {
                        // 15 bits of length
                        let len_bits = self.read_bits(15);
                        let len = Self::val_from_bits(len_bits) as usize;

                        let tlv_bits = self.read_bits(len);

                        let mut tlv = Tlv {
                            data: tlv_bits,
                            cursor: 0,
                        };

                        while tlv.cursor < len {
                            inner_packets.push(tlv.parse());
                        }

                        assert_eq!(len, tlv.cursor);
                    }
                    1 => {
                        // 11 bits of count
                        let count_bits = self.read_bits(11);
                        let count = Self::val_from_bits(count_bits) as usize;

                        for _ in 0..count {
                            inner_packets.push(self.parse());
                        }
                    }
                    _ => unreachable!(),
                }

                PacketData::SubPackets(inner_packets)
            }
        };

        Packet {
            version,
            id,
            packet_data,
        }
    }

    fn parse_literal(&mut self) -> u64 {
        let mut literal_bits: Vec<u8> = vec![];
        loop {
            let b = self.read();

            literal_bits.extend(&self.data[self.cursor..self.cursor + 4]);
            self.cursor += 4;

            if b == 0 {
                break;
            }
        }

        println!("Literal bits {:?}", literal_bits);

        let val = Self::val_from_bits(literal_bits);

        println!("Literal value {}", val);
        val
    }

    fn val_from_bits(literal_bits: Vec<u8>) -> u64 {
        let mut val = 0u64;
        for c in literal_bits {
            val += c as u64;
            val *= 2;
        }
        val /= 2;
        val
    }
}

impl Packet {
    fn sum_versions(&self) -> u64 {
        self.version as u64
            + match &self.packet_data {
                PacketData::Literal(_) => 0u64,
                PacketData::SubPackets(v) => v.iter().map(Packet::sum_versions).sum(),
            }
    }
}

impl crate::lib::DayInner<Day16, i64> for Day16 {
    fn day(&self) -> i32 {
        16
    }

    fn inner(&self, input: String) -> (i64, i64) {
        let mut bit_data: Vec<u8> = vec![];

        for c in input.chars() {
            // Hex parse
            let mut number = u8::from_str_radix(&c.to_string(), 16).unwrap();
            // As binary
            let mut val = 8;
            for _ in 0..4 {
                if number >= val {
                    bit_data.push(1);
                    number -= val;
                } else {
                    bit_data.push(0);
                }

                val /= 2;
            }
        }

        let mut tlv = Tlv {
            data: bit_data,
            cursor: 0,
        };

        println!("{}", tlv);

        let packet = tlv.parse();

        println!("{:?}", packet);

        (packet.sum_versions() as i64, 0)
    }
}
