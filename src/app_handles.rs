use crate::app_config::ApplicationConfig;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};

fn create_input_stream(
    input_file_path: Option<String>,
) -> Result<Box<dyn BufRead>, std::io::Error> {
    Ok(match input_file_path {
        None => Box::new(io::stdin().lock()),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename)?)),
    })
}

fn create_output_stream(
    output_file_path: Option<String>,
) -> Result<Box<dyn Write>, std::io::Error> {
    Ok(match output_file_path {
        None => Box::new(io::stdout().lock()),
        Some(filename) => Box::new(fs::File::create(filename)?),
    })
}

pub struct ApplicationHandles {
    pub rotate_by: i64,
    pub input_handle: Box<dyn BufRead>,
    pub output_handle: Box<dyn Write>,
}

impl ApplicationHandles {
    pub fn from_config(
        app_config: ApplicationConfig,
    ) -> Result<ApplicationHandles, std::io::Error> {
        Ok(ApplicationHandles {
            rotate_by: app_config.rotate_by,
            input_handle: create_input_stream(app_config.input_file_path)?,
            output_handle: create_output_stream(app_config.output_file_path)?,
        })
    }
}
