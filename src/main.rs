// file open 
use std::error::Error; 
use std::io::prelude::*; 
// args
use std::env;

mod shannon{
use std;
use std::fs::File; 
use std::ffi::OsString;
use std::io::prelude::*;

pub struct Shannon {
    filename: OsString,
    filesize: u64,
    freq_table: [u64; 256],
    entropy: f64,
}

impl Shannon {
    pub fn open(path : &OsString) -> Result<Shannon,std::io::Error> {
        //match shannon::Shannon::open(Path::new(&f),f.to_string_lossy().into_owned()){
        let mut file = try!(File::open(path));

        // This doesn't work for pipes
        //let filesize : u64 = try!(fs::metadata(path)).len();
        let mut filesize : u64 = 0;
        let mut freq_table = [0u64; 256];
        let mut buffer = [0; 1024];

        // Read x bytes using a buffer. At EOF, x == 0
        loop {
            let x = try!(file.read(&mut buffer));
            if x == 0 { break; }

            // Process x bytes:
            for &byte in buffer.iter().take(x){
                freq_table[byte as usize] += 1;
            }

            filesize += x as u64;
        }

        let mut entropy : f64 = 0.0;

        for &c in freq_table.iter(){
            if c != 0 {
                let temp : f64 = c as f64 / filesize as f64;
                entropy += -temp * f64::log2(temp);
            }
        }

        //let filename = String::from(path.to_string_lossy().into_owned());
        let filename = path.clone();
        Ok( Shannon { 
            filename: filename,
            filesize: filesize,
            freq_table: freq_table,
            entropy: entropy,
        })
    }
    pub fn most_used_character(&self) -> u8 {
        self.freq_table.iter().enumerate().map(|(x, y)| (y, x)).max().unwrap().1 as u8
    }
    pub fn filename(&self) -> String {
        self.filename.to_string_lossy().into_owned()
    }
    pub fn filesize(&self) -> u64 {
        self.filesize
    }
    pub fn freq_table(&self) -> [u64;256] {
        self.freq_table
    }
    pub fn entropy(&self) -> f64 {
        self.entropy
    }
}

}

fn main() { 
    let args: Vec<_> = env::args_os().skip(1).collect();

    if args.len() == 0 {
        println!("Usage: ./rust-shannon filenames");
        return;
    }

    for f in args{
        match shannon::Shannon::open(&f){
            Err(why) => writeln!(&mut std::io::stderr(), 
                    "couldn't open {}: {}", f.to_string_lossy(), why.description())
                    .expect("failed printing to stderr"),
            Ok(s) => println!("{:.5}\t{}", s.entropy(), s.filename()),
        }
    }
}

#[test]
fn it_works(){
    use shannon::Shannon;
    assert_eq!(Shannon::open("test0").unwrap().entropy(), 0.0);
    assert_eq!(Shannon::open("test1").unwrap().entropy(), 0.0);
    assert_eq!(Shannon::open("test2").unwrap().entropy(), 7.982743005032543);
}

#[test]
#[should_panic]
fn it_panics(){
    let _ent = Shannon::open("").unwrap();
}

#[test]
#[should_panic]
fn it_panics_also(){
    let _ent = Shannon::open("filethatdoenstexist").unwrap();
}
