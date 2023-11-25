use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use structopt::StructOpt;

struct Wc {
    bytes: usize,
    lines: usize,
    words: usize,
    chars: usize,
    file: Option<PathBuf>,
}

impl Wc {
    fn new(file: Option<PathBuf>) -> Wc {
        Wc {
            bytes: 0,
            lines: 0,
            words: 0,
            chars: 0,
            file,
        }
    }

    fn count(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let reader: Box<dyn BufRead> = match &self.file {
            Some(file_path) => Box::new(io::BufReader::new(File::open(file_path)?)),
            None => Box::new(io::BufReader::new(io::stdin())),
        };

        for line in reader.lines() {
            let line = line?;
            self.bytes += line.len();
            self.lines += 1;
            self.words += line.split_whitespace().count();
            self.chars += line.chars().count();
        }

        Ok(())
    }

    fn print(&self, bytes: bool, lines: bool, words: bool, chars: bool) {
        match (bytes, lines, words, chars) {
            (false, false, false, false) => {
                if let Some(filename) = self.get_filename() {
                    println!("{} {} {} {}", self.lines, self.words, self.chars, filename);
                } else {
                    println!("{} {} {}", self.lines, self.words, self.chars);
                }
            }
            _ => {
                if bytes {
                    if let Some(filename) = self.get_filename() {
                        println!("{} bytes, {}", self.bytes, filename);
                    } else {
                        println!("{} bytes", self.bytes);
                    }
                }
                if lines {
                    if let Some(filename) = self.get_filename() {
                        println!("{} lines, {}", self.lines, filename);
                    } else {
                        println!("{} lines", self.lines);
                    }
                }
                if words {
                    if let Some(filename) = self.get_filename() {
                        println!("{} words, {}", self.words, filename);
                    } else {
                        println!("{} words", self.words);
                    }
                }
                if chars {
                    if let Some(filename) = self.get_filename() {
                        println!("{} chars, {}", self.chars, filename);
                    } else {
                        println!("{} chars", self.chars);
                    }
                }
            }
        };
    }

    fn get_filename(&self) -> Option<String> {
        self.file
            .as_ref()
            .map(|path| path.to_string_lossy().into_owned())
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "wc", about = "The UNIX command line tool wc")]
struct Opt {
    /// Prints the number of bytes in a file
    #[structopt(short = "c", long = "bytes")]
    bytes: bool,

    /// Prints the number of lines in a file
    #[structopt(short = "l", long = "lines")]
    lines: bool,

    /// Prints the number of words in a file
    #[structopt(short = "w", long = "words")]
    words: bool,

    /// Print the number of characters in a file
    #[structopt(short = "m", long = "chars")]
    chars: bool,

    /// Sets the input file to use
    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let mut wc = Wc::new(opt.file);
    wc.count()?;
    wc.print(opt.bytes, opt.lines, opt.words, opt.chars);

    Ok(())
}


