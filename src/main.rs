use std::env;
use std::io;
mod chunks;
use chunks::chunks::Chunks;
use termion::color;
use lcs_diff::*;
// https://stackoverflow.com/questions/55555538/what-is-the-correct-way-to-read-a-binary-file-in-chunks-of-a-fixed-size-and-stor/55558173#55558173


fn main() -> io::Result<()> {
    let path_vec: Vec<String> = env::args().take(3).collect();
    let size = 16 as u64;
    let file1 = std::fs::File::open(&path_vec[1])?;
    let file2 = std::fs::File::open(&path_vec[2])?;
    let iter1 = Chunks::new(file1, size as usize);
    let iter2 = Chunks::new(file2, size as usize);
    let chunks = iter1.zip(iter2);
    let mut offset : u64 = 0;
    // : io::Result<Vec<_>>
    chunks.map(|(res_a, res_b)|{
	let a = res_a?;
	let b = res_b?;
	// Ok((a, b))
	let mut left_line = "".to_string();
	let mut right_line = "".to_string();
	for diff in lcs_diff::diff(&a, &b){
	    match diff {
		DiffResult::Common(d) => {
		    left_line += &format!("{:02X}", d.data).to_string();
		    right_line += &format!("{:02X}", d.data).to_string();
		},
		DiffResult::Added(d) => {
		    // left_line +=  "  ";
		    right_line += &format!("{color}{:02x}{clear}", d.data,
					   color=color::Fg(color::Green), clear=color::Fg(color::Reset)).to_string();
		}
		DiffResult::Removed(d) => {
		    left_line += &format!("{color}{:02x}{clear}", d.data,
					  color=color::Fg(color::Red), clear=color::Fg(color::Reset)).to_string();
		    // right_line += "  ";
		}
	    }
	}
	Ok((left_line, right_line))
    }).enumerate().map(|(i, res) : (usize, io::Result<(String, String)>) |{
	let (left, right) : (String, String) = res?;
	io::Result::Ok(format!("{:2X}: {} --- {}", i * size as usize, left, right))
    }).map(|line|{
	Ok(println!("{}", line?))
    }).fold(Ok(()), |prev, res|{
	if prev.is_err() {
	    prev
	} else {
	    res
	}
    })
    // println!("{}", to_print?);

}
