use std::env;
use std::io;
mod chunks;
use chunks::chunks::Chunks;
// use std::io::Read;
// use std::io;
// use std::io::{self, Read, Seek, SeekFrom};


// https://stackoverflow.com/questions/55555538/what-is-the-correct-way-to-read-a-binary-file-in-chunks-of-a-fixed-size-and-stor/55558173#55558173


fn main() -> io::Result<()> {
    let path_vec: Vec<String> = env::args().take(3).collect();
    let size = 16 as u64;
    let file1 = std::fs::File::open(&path_vec[1])?;
    let iter1 = Chunks::new(file1, size as usize); // replace with anything 0xFF was to test
    let file2 = std::fs::File::open(&path_vec[2])?;
    let iter2 = Chunks::new(file2, size as usize); // replace with anything 0xFF was to test
    let chunks = iter1.zip(iter2);
    for (res_a, res_b) in chunks {
	let (mut a, mut b) = (res_a.unwrap(), res_b.unwrap());
	
	let na = a.len();
	let nb = b.len();
	if na == 0 || nb == 0 { break; }
	if na < nb {
	    for _ in 0..nb-na{
		a.push(0)
	    }
	}
	if nb < na {
	    for _ in 0..na-nb{
		b.push(0)
	    }
	}

	// println!("{:?}, {:?}", chunks.len(), chunks.capacity());
	println!("{:02x?} | {:02x?}", a, b);
    }

    Ok(())
}
