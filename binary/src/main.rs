mod convert;
mod add;
mod binary;

fn main() {
    let n = 13;
    println!("{n} ->  {}", convert::binary::demo1(n));
    println!("{n} ->  {}", convert::binary::demo2(n));
    println!("{n} ->  {}", convert::binary::demo3(n));
    println!("{n} ->  {}", convert::binary::demo4(n));
    println!("{n} ->  {}", convert::hexa::demo1(n));
    println!("{n} ->  {}", convert::hexa::demo2(n));
    println!("{n} ->  {}", convert::hexa::demo3(n));

    let l = 0b1011;
    let r = 0b0101;
    println!("{l} + {r} => {}", add::binary::demo1(l, r));
    println!("{l} + {r} => {}", add::binary::demo2(l, r));
    println!("{l} + {r} => {}", add::binary::demo3("1011", "0101"));

    println!("mask(1010) -> {:b}", binary::mask(0b1010));

    println!("shift_left(1010) -> {:b}", binary::mask(binary::shift_left(0b1010)));

    println!("shift_right(1010) -> {:b}", binary::mask(binary::shift_rigtt(0b1010)));

    println!("big_endian(1010) -> '{:016b}'", binary::big_endian(0b1111111100000000));

    println!("little_endian(1010) -> '{:016b}'", binary::little_endian(0b1111111100000000));

    let b = 0b1111111100000000;
    println!("low(1111111100000000) -> {:08b}", binary::low(b));
    println!("high(1111111100000000) -> {:08b}", binary::high(b));
}

