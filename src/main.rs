use crossterm::{cursor, execute, style::Print, terminal};

type Location = (u16, u16);

struct Dimension {
    width: u16,
    height: u16,

    columns: u16,
    rows: u16,
}

trait Tag {
    fn display(&self, parent_dimension: &Dimension, parent_position: &Location);
}

enum Position {
    Left,
    Centre,
    Right,
    AbsoluteLeft,
    AbsoluteCentre,
    AbsoluteRight,
    Relative(Location),
    Absolute(Location),
}

struct Placement {
    vertical: Position,
    horizontal: Position,
}

struct FontWeight {
    bold: bool,
    italics: bool,
    underline: bool,
}

struct Style {
    pos: Placement,
    font_weight: FontWeight,
}

struct Page {
    screen_size: Dimension,
    children: Vec<Box<dyn Tag>>,
}

struct Text {
    value: String,
    style: Style,
}

fn main() {
    centre_text("hello world");
}

fn centre_text(msg: &str) {
    for i in msg.split('\n') {
        print_centre(i);
    }
}

fn print_centre(msg: &str) {
    let (_, cursor_row) = cursor::position().unwrap();
    let start = terminal::window_size().map_or(0, |x| {
        (x.columns / 2)
            .checked_sub(msg.len() as u16 / 2)
            .unwrap_or_default()
    });

    execute!(
        std::io::stdout(),
        cursor::MoveTo(start, cursor_row),
        Print(msg),
        Print('\n')
    )
    .unwrap();
}
