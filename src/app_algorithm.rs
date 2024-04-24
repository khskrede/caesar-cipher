use std::io::{BufRead, Write};

pub fn do_rotate(
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
