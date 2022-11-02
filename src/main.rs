
use termion;
use termion::event::{Key, Event};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn search () {
}

fn main() {

    let left = termion::cursor::Left(1);

    let mut needle: String = "".to_owned();

    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Backspace) => { _ = needle.pop(); _ = write!(stdout, "{} {}", left, left); search() },
            Event::Key(Key::Char('\n')) => { }
            Event::Key(Key::Char(x)) => { needle.push(x); _ = write!(stdout, "{}", x); search() },
            Event::Key(Key::Esc) => { break; },
            _ => { }
        }
        stdout.flush().unwrap();
    }
}

/*fn main() {
    let mut v = vec![];

    for file in fs::read_dir("/home/sunflower").unwrap() {
        let path = file.unwrap().path();
        v.push(path.to_str().unwrap().to_owned());
    }

    println!("{:#?}", v)


}
*/