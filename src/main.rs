use crate::app_algorithm::do_rotate;
use crate::app_config::ApplicationConfig;
use crate::app_handles::ApplicationHandles;

mod app_algorithm;
mod app_config;
mod app_handles;

fn apply_rotate(mut handles: ApplicationHandles) -> Result<(), std::io::Error> {
    do_rotate(
        &mut handles.input_handle,
        &mut handles.output_handle,
        handles.rotate_by,
    )
}

fn main() -> Result<(), std::io::Error> {
    let app_config = ApplicationConfig::from_args();
    let app_handles = ApplicationHandles::from_config(app_config)?;
    apply_rotate(app_handles)
}
