use std::{env, io};
use std::fs::File;

use serde_json::{map::Map, Value, from_slice};
use main_error::MainError;
use memmap::Mmap;

fn main() -> Result<(), MainError> {
    let filename = env::args().nth(1).expect("json filename missing");
    let file = File::open(filename)?;
    let memory = unsafe { Mmap::map(&file)? };

    let array: Vec<Map<String, Value>> = from_slice(&memory)?;
    let fields: Vec<String> = array[0].keys().cloned().collect();

    let mut wtr = csv::Writer::from_writer(io::stdout());
    wtr.write_record(&fields)?;

    for object in array {
        for field in &fields {
            match object.get(field).and_then(|v| v.as_str()) {
                Some(string) => wtr.write_field(string)?,
                None => wtr.write_field("")?,
            }
        }
        wtr.write_record(None::<&[u8]>)?;
    }

    wtr.flush()?;

    Ok(())
}
