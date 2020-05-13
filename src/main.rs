use std::fs::File;
use std::env;
use std::io::BufReader;
// use std::io::Read;
// use std::io;
use std::io::{self, Read, Seek, SeekFrom};


// https://stackoverflow.com/questions/55555538/what-is-the-correct-way-to-read-a-binary-file-in-chunks-of-a-fixed-size-and-stor/55558173#55558173



// fn main() -> io::Result<()>{
//     // let args = ;
//     // let path2 = args.take(1);
// let path_vec: Vec<String> = env::args().take(3).collect();
// let mut file1 = File::open(&path_vec[1])?;
// let mut file2 = File::open(&path_vec[2])?;
// let line_len = 16 as u64;
//     // let mut buf_reader1 = BufReader::new(file1);
//     // let mut buf_reader2 = BufReader::new(file2);
//     // buf_reader1.by_ref().take(line_len).read_to_end(&mut buf1);
//     // buf_reader2.by_ref().take(line_len).read_to_end(&mut buf2);
//     let mut buf1 = Vec::with_capacity(line_len as usize);
//     let mut buf2 = Vec::with_capacity(line_len as usize);
//     while let (Ok(n1), Ok(n2)) =
// 	( file1.by_ref().take(line_len).read_to_end(&mut buf1),
// 	  file2.by_ref().take(line_len).read_to_end(&mut buf2) ) {
// 	    if n1 == 0 || n2 == 0 { break; }
// 	    if n1 < n2 {
// 		for _ in 0..n2-n1{
// 		    buf1.push(0)
// 		}
// 	    }
// 	    if n2 < n1 {
// 		for _ in 0..n1-n2{
// 		    buf2.push(0)
// 		}
// 	    }
// 	    // println!("{:^width$x?} | {:^width$x?}", buf1, buf2,
// 	    println!("{:02x?} | {:02x?}", buf1, buf2);
// 	    buf1 = buf1.into_iter().skip(n1).collect();
// 	    buf2 = buf2.into_iter().skip(n2).collect();
// 	}
//     Ok(())
// }



fn main() -> io::Result<()> {
    let path_vec: Vec<String> = env::args().take(3).collect();
    let size = 16 as u64;
    let file1 = std::fs::File::open(&path_vec[1])?;
    let iter1 = Chunks::from_seek(file1, size as usize)?; // replace with anything 0xFF was to test
    let file2 = std::fs::File::open(&path_vec[2])?;
    let iter2 = Chunks::from_seek(file2, size as usize)?; // replace with anything 0xFF was to test

    // println!("{:?}", iter.size_hint());
    // This iterator could return Err forever be careful collect it into an Result
    // let chunks1 = iter1.collect::<Result<Vec<_>, _>>()?;
    // let chunks2 = iter2.collect::<Result<Vec<_>, _>>()?;
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
