use std::io::{self, BufRead, BufReader, Write};
use std::fs;
use clap::Parser;

fn do_rotate(
    input_stream: &mut dyn BufRead,
    output_stream: &mut dyn Write,
    rotate_by: i64,
) -> std::result::Result<(), std::io::Error> {
    loop {
        let buf = input_stream.fill_buf()?;

        if buf.is_empty() {
            return Result::Ok(());
        }

        let read_bytes_num = buf.len();

        let to_write: Vec<u8> = buf.iter().map(|x| (*x as i64 + rotate_by) as u8).collect();

        output_stream.write_all(to_write.as_slice())?;
        input_stream.consume(read_bytes_num);
    }
}

#[derive(Parser)]
struct Arguments {
    rotate_by : i64,

    #[clap(short, long)]
    input_file_path : Option<String>,

    #[clap(short, long)]
    output_file_path : Option<String>,
}

fn main() -> Result<(), std::io::Error> {
    let args = Arguments::parse();

    let mut in_handle: Box<dyn BufRead> = match args.input_file_path {
        None => Box::new(io::stdin().lock()),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename)?)),
    };

    let mut out_handle: Box<dyn Write> = match args.output_file_path {
        None => Box::new(io::stdout().lock()),
        Some(filename) => Box::new(fs::File::create(filename)?),
    };

    do_rotate(&mut in_handle, &mut out_handle, args.rotate_by).map(|_| ())
}

#[cfg(test)]
mod tests {
    use crate::do_rotate;

    #[test]
    fn test_do_rotate() {
        let mut test_input: &[u8] = b"abcdefg".as_slice();
        let mut test_output: Vec<u8> = Vec::new();

        let expected: Vec<u8> = Vec::from(b"bcdefgh");

        _ = do_rotate(&mut test_input, &mut test_output, 1);

        assert_eq!(test_output, expected);
    }
}
