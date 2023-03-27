mod generic_writer;

fn main() {
    let mut reader = "Hello, world!".as_bytes();
    let mut writer = Vec::new();

    generic_writer::copy(&mut reader, &mut writer).unwrap();
}

