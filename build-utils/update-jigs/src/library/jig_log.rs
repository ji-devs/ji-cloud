use std::fs::File;
use std::io::Write;
use scan_fmt::scan_fmt;

pub struct JigInfoLogLine {
    pub jig_id: String,
    pub game_id: String,
    pub game_hash: String
}

impl JigInfoLogLine {
    pub fn read_line(line: &str) -> Self {
        let (jig_id, game_id, game_hash) = scan_fmt!(&line, "{} {} {}", String, String, String).unwrap();
        Self {
            jig_id,
            game_id,
            game_hash
        }
    }

    pub fn write_line<W: Write>(self: &Self, mut output: W) {
        writeln!(output, "{} {} {}", self.jig_id, self.game_id, self.game_hash).unwrap();
    }
}