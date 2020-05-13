

pub mod chunks{
    use std::io::{self, Read};
    pub struct Chunks<R> {
	read: R,
	size: usize,
    }

    impl<R> Chunks<R> {
	pub fn new(read: R, size: usize) -> Self {
            Self {
		read,
		size,
            }
	}

	// This could be useful if you want to try to recover from an error
	pub fn into_inner(self) -> R {
            self.read
	}
    }

    impl<R> Iterator for Chunks<R>
    where
	R: Read,
    {
	type Item = io::Result<Vec<u8>>;

	fn next(&mut self) -> Option<Self::Item> {
            let mut chunk = Vec::with_capacity(self.size);
            match self
		.read
		.by_ref()
		.take(chunk.capacity() as u64)
		.read_to_end(&mut chunk)
            {
		Ok(n) => {
                    if n != 0 {
			Some(Ok(chunk))
                    } else {
			None
                    }
		}
		Err(e) => Some(Err(e)),
            }
	}
    }

    trait ReadPlus: Read {
	fn chunks(self, size: usize) -> Chunks<Self>
	where
            Self: Sized,
	{
            Chunks::new(self, size)
	}
    }

    impl<T: ?Sized> ReadPlus for T where T: Read {}
}
