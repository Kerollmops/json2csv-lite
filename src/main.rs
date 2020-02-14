use std::io;
use std::borrow::Cow;

use main_error::MainError;
use serde_json::{map::Map, de::IoRead, Value, StreamDeserializer};

fn value_to_string(value: &Value) -> Option<Cow<str>> {
    match value {
        Value::Null => None,
        Value::Bool(b) => Some(Cow::Owned(b.to_string())),
        Value::Number(n) => Some(Cow::Owned(n.to_string())),
        Value::String(s) => Some(Cow::Borrowed(s.as_ref())),
        Value::Array(_) => None,
        Value::Object(_) => None,
    }
}

fn main() -> Result<(), MainError> {
    let stdin = IoRead::new(io::stdin());
    let mut stream = StreamDeserializer::<_, Map<String, Value>>::new(stdin).peekable();

    let first = stream.peek().unwrap().as_ref().unwrap().clone();
    let fields: Vec<_> = first.keys().collect();

    let mut wtr = csv::Writer::from_writer(io::stdout());
    wtr.write_record(&fields)?;

    for object in stream {
        let object = object?;
        for field in &fields {
            match object.get(field.as_str()).and_then(value_to_string) {
                Some(string) => wtr.write_field(string.as_ref())?,
                None => wtr.write_field("")?,
            }
        }
        wtr.write_record(None::<&[u8]>)?;
    }

    wtr.flush()?;

    Ok(())
}
