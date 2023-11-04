#![allow(unused_variables)]

use std::fmt;
use std::fmt::Display;

// ! 한 번에 한 단계씩 파일을 시뮬레이트
//
// // maybe file system
#[derive(Default, Debug, PartialEq)]
enum FileState {
    #[default]
    Closed,
    Open,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

#[derive(Debug, Default)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    // create new file
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            ..Default::default()
        }
    }
    // read file
    fn read(
        self: &File,
        save_to: &mut Vec<u8>
        ) -> Result<usize, String> {

        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }

        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}


// open file
fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

// close file
fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}


fn main() {
    let mut f = File::new("1.txt");
    let mut buffer: Vec<u8> = vec![];

    if f.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }

    f = open(f).unwrap();
    let f_length = f.read(&mut buffer).unwrap();
    f = close(f).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{}", f);
    println!("{:?}", f);
}
