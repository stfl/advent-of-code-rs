use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub fn read_input_lines_from_path(input_path: impl AsRef<Path>) -> impl Iterator<Item = String> {
    let f = File::open(input_path).unwrap();
    let b = BufReader::new(f);
    b.lines().map(|l| l.expect("Bad line!"))
}

#[macro_export]
macro_rules! read_input_lines {
    //reads the input.txt file inside the challenge crate root
    //this needs to be called from main.rs because it finds input.txt relative to the main.rs file
    () => {{
        let input_path = Path::new(file!())
            .parent()
            .unwrap()
            .join(Path::new("../input.txt"));
        read_input::read_input_lines_from_path(input_path)
    }};
}
