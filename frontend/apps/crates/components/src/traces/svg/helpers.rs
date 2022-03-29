use shared::domain::jig::module::body::_groups::design::PathCommand;
use std::fmt::Write;
use utils::prelude::*;

pub fn path_command_to_string(path: impl Iterator<Item = (PathCommand, bool)>) -> String {
    let mut output = String::new();

    for (index, (command, absolute)) in path.enumerate() {
        if index != 0 {
            output.push(' ')
        }
        let prefix = match &command {
            PathCommand::MoveTo(_, _) => {
                if absolute {
                    "M"
                } else {
                    "m"
                }
            }
            PathCommand::ClosePath => {
                if absolute {
                    "Z"
                } else {
                    "z"
                }
            }
            PathCommand::LineTo(_, _) => {
                if absolute {
                    "L"
                } else {
                    "l"
                }
            }
            PathCommand::HorizontalLineTo(_) => {
                if absolute {
                    "H"
                } else {
                    "h"
                }
            }
            PathCommand::VerticalLineTo(_) => {
                if absolute {
                    "V"
                } else {
                    "v"
                }
            }
            PathCommand::CurveTo(_, _, _, _, _, _) => {
                if absolute {
                    "C"
                } else {
                    "c"
                }
            }
            PathCommand::SmoothCurveTo(_, _, _, _) => {
                if absolute {
                    "S"
                } else {
                    "s"
                }
            }
            PathCommand::QuadCurveTo(_, _, _, _) => {
                if absolute {
                    "Q"
                } else {
                    "q"
                }
            }
            PathCommand::SmoothQuadCurveTo(_, _) => {
                if absolute {
                    "T"
                } else {
                    "t"
                }
            }
            PathCommand::ArcTo(_, _, _, _, _, _, _) => {
                if absolute {
                    "A"
                } else {
                    "a"
                }
            }
        };

        output.push_str(prefix);

        match command {
            PathCommand::MoveTo(x, y) => {
                write!(&mut output, "{} {}", x, y).unwrap_ji();
            }
            PathCommand::ClosePath => {}
            PathCommand::LineTo(x, y) => {
                write!(&mut output, "{} {}", x, y).unwrap_ji();
            }
            PathCommand::HorizontalLineTo(x) => {
                write!(&mut output, "{}", x).unwrap_ji();
            }
            PathCommand::VerticalLineTo(y) => {
                write!(&mut output, "{}", y).unwrap_ji();
            }
            PathCommand::CurveTo(cp1x, cp1y, cp2x, cp2y, x, y) => {
                write!(
                    &mut output,
                    "{} {} {} {} {} {}",
                    cp1x, cp1y, cp2x, cp2y, x, y
                )
                .unwrap_ji();
            }
            PathCommand::SmoothCurveTo(cp1x, cp1y, x, y) => {
                write!(&mut output, "{} {} {} {}", cp1x, cp1y, x, y).unwrap_ji();
            }
            PathCommand::QuadCurveTo(cp1x, cp1y, x, y) => {
                write!(&mut output, "{} {} {} {}", cp1x, cp1y, x, y).unwrap_ji();
            }
            PathCommand::SmoothQuadCurveTo(x, y) => {
                write!(&mut output, "{} {}", x, y).unwrap_ji();
            }
            PathCommand::ArcTo(a, b, c, d, e, f, g) => {
                write!(&mut output, "{} {} {} {} {} {} {}", a, b, c, d, e, f, g).unwrap_ji();
            }
        }
    }

    output
}

pub fn path_to_string(path: impl Iterator<Item = (f64, f64)>) -> String {
    let mut count = 0;
    let mut output = String::from("M");
    for (_index, (x, y)) in path.enumerate() {
        write!(&mut output, " {} {}", x, y).unwrap_ji();
        count += 1;
    }

    output.push_str(" Z");

    if count < 2 {
        String::from("M 0 0")
    } else {
        output
    }
}
