use std::fs::File;
use std::io::{Read, Seek};

#[derive(Debug)]
struct ELF {
    magic: [u8; 4],
    class: String,
    machine: String,
}

fn read(path: &str) -> ELF {
    let mut file = File::open(path).unwrap();
    let mut magic = [0u8; 4];
    file.read_exact(&mut magic).unwrap();
    if magic != [0x7F, b'E', b'L', b'F'] {
        panic!("Not an ELF file");
    }
    file.seek(std::io::SeekFrom::Start(0)).unwrap();
    let mut e_ident = [0u8; 16];
    let mut e_type = [0u8; 2];
    let mut e_machine = [0u8; 2];
    file.read_exact(&mut e_ident).unwrap();
    file.read_exact(&mut e_type).unwrap();
    file.read_exact(&mut e_machine).unwrap();

    ELF {
        magic,
        class: match e_ident[4] {
            1 => "32-bit".to_string(),
            2 => "64-bit".to_string(),
            _ => panic!("Unknown class"),
        },
        machine: match u16::from_le_bytes(e_machine) {
            3 => "x86".to_string(),
            62 => "x86-64".to_string(),
            40 => "ARM".to_string(),
            183 => "Aarch64".to_string(),
            _ => format!("Unknown machine: {}", e_machine[0]),
        },
    }
}

fn detect_endianness() {
    let n: u16 = 1;
    let bytes = n.to_ne_bytes(); // native endian

    if bytes[0] == 1 {
        println!("little endian");
    } else {
        println!("big endian");
    }
}

fn main() {
    detect_endianness();
    let path = "target/release/elf";
    let info = read(path);
    println!("{:?}", info)
}
