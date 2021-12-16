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
    fn parse(&mut self) -> Packet {
        // Read first three bits (V)
        let v_bits = &self.data[self.cursor..self.cursor + 3];
        let mut version = 0;
        println!("V={:?}", v_bits);
        let mut val = 4;
        for v in v_bits.iter().take(3) {
            if *v == 1 {
                version += val;
            }
            val /= 2;
            self.cursor += 1;
        }

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
            _ => unreachable!(),
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
            let b = self.data[self.cursor];
            self.cursor += 1;

            literal_bits.extend(&self.data[self.cursor..self.cursor + 4]);
            self.cursor += 4;

            if b == 0 {
                break;
            }
        }

        println!("Literal bits {:?}", literal_bits);

        let mut val = 0u64;
        for c in literal_bits {
            val += c as u64;
            val *= 2;
        }
        val /= 2;

        println!("Literal value {}", val);
        val
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

        (0, 0)
    }
}
