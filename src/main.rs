use std::env;
use std::io::{self, BufRead, Write};

fn do_rotate<Input: BufRead, Output: Write>(
    input_stream: &mut Input,
    output_stream: &mut Output,
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

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let mut in_handle = io::stdin().lock();
    let mut out_handle = io::stdout().lock();

    args[1]
        .parse::<i64>()
        .map_err(|t| std::io::Error::new(io::ErrorKind::InvalidInput, t))
        .and_then(|rotate_by| do_rotate(&mut in_handle, &mut out_handle, rotate_by).map(|_| ()))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    #[test]
    fn test_do_rotate() {
        // TODO
        assert_eq!(1, 1)
    }
}
