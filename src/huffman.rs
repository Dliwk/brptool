// This file is basically huffman.cc from original Ballistica source, but written in Rust

const FREQS: [u32; 256] = [
    101342, 9667, 3497, 1072, 0, 3793, 0, 0, 2815, 5235, 0, 0, 0, 3570, 0, 0, 0, 1383, 0, 0, 0,
    2970, 0, 0, 2857, 0, 0, 0, 0, 0, 0, 0, 0, 1199, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1494, 1974, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1351, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1475, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

macro_rules! precondition {
    ($condition:expr) => {
        if !$condition {
            panic!("precondition failed: {}", stringify!($condition));
        }
    };
}

#[derive(Default, Debug, Copy, Clone)]
struct Node {
    id: usize,
    left_child: Option<usize>,
    right_child: Option<usize>,
    parent: Option<usize>,
    bits: u8,
    val: u16,

    frequency: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Huffman {
    nodes: [Node; 511],
}

impl Huffman {
    pub fn build() -> Self {
        let mut nodes = Vec::<Node>::with_capacity(511);
        for i in 0..256 {
            nodes.push(Node {
                frequency: FREQS[i],
                id: i,
                ..Node::default()
            });
        }

        while nodes.len() < 511 {
            let mut smallest = (0, 0);
            // let smallest: (_, _) = {
            //     let mut sorted: Vec<_> = nodes.iter().filter(|x| x.parent.is_none()).collect();
            //     sorted.sort_by_key(|x| x.frequency);
            //     (sorted[0].id, sorted[1].id)
            // };
            // XXX: We're implementing *literally* the same algorithm which is used in huffman.cc,
            //      because otherwise it can generate different binary tree = not compatible
            //      encoding/decoding.
            let mut i = 0;
            while nodes[i].parent.is_some() {
                i += 1;
            }
            smallest.0 = i;
            i += 1;
            while nodes[i].parent.is_some() {
                i += 1;
            }
            smallest.1 = i;
            i += 1;
            while i < nodes.len() {
                if nodes[i].parent.is_none() {
                    if nodes[smallest.0].frequency > nodes[smallest.1].frequency {
                        if nodes[i].frequency < nodes[smallest.0].frequency {
                            smallest.0 = i;
                        }
                    } else {
                        if nodes[i].frequency < nodes[smallest.1].frequency {
                            smallest.1 = i;
                        }
                    }
                }
                i += 1;
            }

            let node = Node {
                id: nodes.len(),
                frequency: nodes[smallest.0].frequency + nodes[smallest.1].frequency,
                right_child: Some(smallest.0),
                left_child: Some(smallest.1),
                ..Node::default()
            };
            nodes[smallest.0].parent = Some(node.id);
            nodes[smallest.1].parent = Some(node.id);
            nodes.push(node);
        }
        assert!(nodes[509].parent.is_some());
        assert!(nodes[510].parent.is_none());

        for we in 0..256 {
            let mut index = we;

            // 1 if left child, 0 if right child
            while let Some(par) = nodes[index].parent.clone() {
                if index == nodes[par].left_child.unwrap() {
                    nodes[we].val = nodes[we].val << 1 | 0x01;
                } else {
                    nodes[we].val = nodes[we].val << 1;
                }
                nodes[we].bits += 1;
                index = par;
            }

            let our = &mut nodes[we];
            if our.bits >= 8 {
                our.bits = 8;
                our.val = our.val << 1;
            } else {
                our.val = our.val << 1 | 0x01;
            }
            our.bits += 1;
        }

        Huffman {
            nodes: nodes.try_into().unwrap(),
        }
    }
    pub fn decompress(&self, src: &[u8]) -> Vec<u8> {
        let length = src.len();
        precondition!(length > 0);

        let remainder = src[0] & 0x0F;
        let compressed = (src[0] >> 7) != 0;

        if compressed {
            let mut out = Vec::with_capacity(length * 2);

            let bit_length = (length - 1) * 8;
            if remainder as usize > bit_length {
                panic!("invalid huffman data");
            }
            let bit_length = bit_length - remainder as usize;
            let mut bit = 0;
            let ptr = &src[1..];

            while bit < bit_length {
                let bitval = (ptr[bit / 8] >> (bit % 8)) & 0x01;
                bit += 1;

                if bitval == 1 {
                    let val: u8;
                    let mut n = 510;
                    precondition!(self.nodes[n].parent.is_none());
                    loop {
                        precondition!(n <= 510);

                        let bitval = (ptr[bit / 8] >> (bit % 8)) & 0x01;

                        // 1 for right, 0 for left
                        if bitval == 0 {
                            if let Some(node_id) = self.nodes[n].left_child {
                                n = node_id;
                                bit += 1;
                            } else {
                                val = n as u8;
                                break;
                            }
                        } else {
                            if let Some(node_id) = self.nodes[n].right_child {
                                n = node_id;
                                bit += 1;
                            } else {
                                val = n as u8;
                                break;
                            }
                        }

                        if self.nodes[n].left_child.is_none() && self.nodes[n].right_child.is_none()
                        {
                            val = n as u8;
                            break;
                        }

                        if bit > bit_length {
                            panic!("huffman decompress got bit > bit_length");
                        }
                    }
                    out.push(val);
                } else {
                    // just read next 8 bits as value
                    let val: u8;
                    if bit % 8 == 0 {
                        precondition!((bit / 8) < (length - 1));
                        val = ptr[bit / 8];
                    } else {
                        precondition!((bit / 8 + 1) < (length - 1));
                        val = ((ptr[bit / 8]) >> bit % 8) | ((ptr[bit / 8 + 1]) << (8 - bit % 8));
                    }
                    out.push(val);
                    bit += 8;
                    if bit > bit_length {
                        panic!("huffman decompress got bit > bit_length b");
                    }
                }
            }

            precondition!(bit == bit_length);
            out
        } else {
            src.into()
        }
    }
}
