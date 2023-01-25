use std::{io, thread};
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut i = trust::Interface::new()?;
    eprintln!("created interface");
    let mut l1 = i.bind(8000)?;
    let jh1 = thread::spawn(move || {
        while let Ok(mut stream) = l1.accept() {
            eprintln!("got connection!");
            let n = stream.read(&mut [0]).unwrap();
            eprintln!("read data");
            assert_eq!(n, 0);
        }
    });
    jh1.join().unwrap();

    Ok(())
}
