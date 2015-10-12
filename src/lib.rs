use std::char;
use std::cmp;
use std::cmp::{min,max};

const UP:isize    = -1;
const DOWN:isize  =  1;
const LEFT:isize  = -1;
const RIGHT:isize =  1;

const MOVEMENTS:[(isize, isize);4] = [
	(UP, LEFT),
	(UP, RIGHT),
	(DOWN, LEFT),
	(DOWN, RIGHT),
];

pub struct Mode<'a> {
	pub height:   usize,
	pub width:    usize, 
	pub alphabet: &'a [u8],
}

pub const OPENSSL: Mode<'static> = Mode {
	height: 9,
	width: 17,
	alphabet: b" .o+=*BOX@%&#/^SE",
};

fn in_range<T: cmp::Ord>(minumum: T, value: T, maximum: T) -> T
{
	min(max(minumum, value), maximum)
}

pub fn drunken_bishop<'a>(fingerprint: &'a [u8], mode: Mode) -> String {
	let mut field = vec![vec![0; mode.width]; mode.height];

	let mut pos = (mode.height/2, mode.width/2);
	for byte in fingerprint {
		for i in 0..4 {
			let code = (*byte as usize & (0b11 << (2*i))) >> (2*i);
			let (di,dj) = MOVEMENTS[code];

			pos.0 = in_range(0, pos.0 as isize + di, mode.height as isize -1) as usize;
			pos.1 = in_range(0, pos.1 as isize + dj, mode.width as isize -1) as usize;

			field[pos.0][pos.1] += 1;
		}
	}

	field[mode.height/2][mode.width/2] = mode.alphabet.len()-2;
	field[pos.0][pos.1] = mode.alphabet.len()-1;

	let mut res = String::new();
	for row in field {
		res.push('|');
		for val in row {
			let c = char::from_u32(mode.alphabet[min(val, mode.alphabet.len()-1)] as u32);
			res.push(c.unwrap());
		}
		res.push_str("|\n");
	}

	format!("+{1}+\n{0}+{1}+\n", res, (0..mode.width).map(|_| "-").collect::<String>())
}

#[test]
fn test1() {
	let input = [0x16, 0x27, 0xac, 0xa5, 0x76, 0x28, 0x2d, 0x36, 0x63, 0x1b, 0x56, 0x4d, 0xeb, 0xdf, 0xa6, 0x48];

	let expected = "+-----------------+\n\
	                |        .        |\n\
					|       + .       |\n\
					|      . B .      |\n\
					|     o * +       |\n\
					|    X * S        |\n\
					|   + O o . .     |\n\
					|    .   E . o    |\n\
					|       . . o     |\n\
					|        . .      |\n\
					+-----------------+\n";

	println!("'{}'", expected);
	println!("'{}'", drunken_bishop(&input[..], OPENSSL));

	assert_eq!(drunken_bishop(&input[..], OPENSSL), expected);
}

#[test]
fn test2() {
	let input = [0xb6, 0xdd, 0xb7, 0x1f, 0xbc, 0x25, 0x31, 0xd3, 0x12, 0xf4, 0x92, 0x1c, 0x0b, 0x93, 0x5f, 0x4b];

	let expected = "+-----------------+\n\
	                |            o.o  |\n\
	                |            .= E.|\n\
	                |             .B.o|\n\
	                |              .= |\n\
	                |        S     = .|\n\
	                |       . o .  .= |\n\
	                |        . . . oo.|\n\
	                |             . o+|\n\
	                |              .o.|\n\
	                +-----------------+\n";

	println!("'{}'", expected);
	println!("'{}'", drunken_bishop(&input[..], OPENSSL));

	assert_eq!(drunken_bishop(&input[..], OPENSSL), expected);
}

#[test]
fn test3() {
	let input = [0x05, 0x1e, 0x1e, 0xc1, 0xac, 0xb9, 0xd1, 0x1c, 0x6a, 0x60, 0xce, 0x0f, 0x77, 0x6c, 0x78, 0x47];

	let expected = "+-----------------+\n\
	                |       o=.       |\n\
	                |    o  o++E      |\n\
	                |   + . Ooo.      |\n\
	                |    + O B..      |\n\
	                |     = *S.       |\n\
	                |      o          |\n\
	                |                 |\n\
	                |                 |\n\
	                |                 |\n\
	                +-----------------+\n";

	println!("'{}'", expected);
	println!("'{}'", drunken_bishop(&input[..], OPENSSL));

	assert_eq!(drunken_bishop(&input[..], OPENSSL), expected);
}

#[test]
fn test4() {
	let input = [0x17, 0xcd, 0xe2, 0xab, 0x1a, 0x4b, 0x7d, 0x97, 0x89, 0xd9, 0xc3, 0x7b, 0xb9, 0x12, 0x08, 0x48];

	let expected = "+-----------------+\n\
	                |                 |\n\
	                |      E    o     |\n\
	                |     . .  o o    |\n\
	                |      . .. o     |\n\
	                |        S.o.     |\n\
	                |       . ..*.o   |\n\
	                |      o . = B. . |\n\
	                |     . o o ..oo  |\n\
	                |      o..   .o.. |\n\
	                +-----------------+\n";

	println!("'{}'", expected);
	println!("'{}'", drunken_bishop(&input[..], OPENSSL));

	assert_eq!(drunken_bishop(&input[..], OPENSSL), expected);
}

