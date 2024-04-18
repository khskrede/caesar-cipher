use clap::{Args, Parser, Subcommand};
use std::fs;
use std::io::{self, BufRead, BufReader, Write};

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
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Encrypt input data using key
    Encrypt {
        key: i64,
        input_file_path: Option<String>,
        output_file_path: Option<String>,
    },

    /// Decrypt input data using key
    Decrypt {
        key: i64,
        input_file_path: Option<String>,
        output_file_path: Option<String>,
    },
}

#[derive(Args)]
struct Arguments {
    rotate_by: i64,

    #[clap(short, long)]
    input_file_path: Option<String>,

    #[clap(short, long)]
    output_file_path: Option<String>,
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();

    let (rotate_by, input_file_path, output_file_path) = match args.command {
        Command::Encrypt {
            key,
            input_file_path,
            output_file_path,
        } => (key, input_file_path, output_file_path),
        Command::Decrypt {
            key,
            input_file_path,
            output_file_path,
        } => (-key, input_file_path, output_file_path),
    };

    let mut in_handle: Box<dyn BufRead> = match input_file_path {
        None => Box::new(io::stdin().lock()),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename)?)),
    };

    let mut out_handle: Box<dyn Write> = match output_file_path {
        None => Box::new(io::stdout().lock()),
        Some(filename) => Box::new(fs::File::create(filename)?),
    };

    do_rotate(&mut in_handle, &mut out_handle, rotate_by).map(|_| ())
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

    #[test]
    fn test_do_rotate_with_negative() {
        let mut test_input: &[u8] = b"bcdefgh".as_slice();
        let mut test_output: Vec<u8> = Vec::new();

        let expected: Vec<u8> = Vec::from(b"abcdefg");
        _ = do_rotate(&mut test_input, &mut test_output, -1);

        assert_eq!(test_output, expected);
    }

    // Write a test that operates on non-alphanumeric characters
    #[test]
    fn test_do_rotate_non_alphanum() {
        let mut test_input: &[u8] = [0, 255].as_slice();
        let mut test_output: Vec<u8> = Vec::new();

        let expected: Vec<u8> = Vec::from([255, 254].as_slice());

        _ = do_rotate(&mut test_input, &mut test_output, -1);

        assert_eq!(test_output, expected);
    }

    // Write a test that operates on empty input
    #[test]
    fn test_do_rotate_empty_input() {
        let mut test_input: &[u8] = [].as_slice();
        let mut test_output: Vec<u8> = Vec::new();

        let expected: Vec<u8> = Vec::new();

        _ = do_rotate(&mut test_input, &mut test_output, 1);

        assert_eq!(test_output, expected);
    }
}
