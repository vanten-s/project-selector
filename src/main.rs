
use termion;
use termion::event::{Key, Event};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use std::fs;

fn search (param: &str) -> Vec<String>
{
    let mut haystack = vec![];

    for file in fs::read_dir("/home/sunflower").unwrap() {
        let path = file.unwrap().path();
        let s: String = path.to_str().unwrap().to_owned();
        if s.contains(param) {
            haystack.push(s);
        }
    }

    haystack


}

fn wr (needle: &str, haystack: Vec<String>)
{

    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());


    _ = write!(stdout, "{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), needle);
    
    let mut i = 2;
    
    for hay in haystack {
        _ = write!(stdout, "{}{}", termion::cursor::Goto(1, i), hay);
        i += 1;
    }    
    
    _ = write!(stdout, "{}", termion::cursor::Goto(1, 1));

    _ = stdout.flush();
}

fn main() 
{

    
    let mut _stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    wr("", vec!["".to_owned()]);

    let mut needle: String = "".to_owned();

    let mut res;

    let stdin = stdin();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Backspace) => { 
                _ = needle.pop(); 
                res = search(&needle);
                wr(&needle, res);
            },
            Event::Key(Key::Char('\n')) => { },
            Event::Key(Key::Char(x)) => { 
                needle.push(x); 
                res = search(&needle);
                wr(&needle, res);
            },
            Event::Key(Key::Esc) => { break; },
            _ => { }
        }
    }
}
