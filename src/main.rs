
use termion;
use termion::event::{Key, Event};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use termion::color;
use std::io::{Write, stdout, stdin};
use std::fs;
use std::env;
use std::process::Command;
use dirs;
use json;

fn get_dir() -> String 
{
    json::parse(&fs::read_to_string(dirs::config_dir().unwrap().to_str().unwrap().to_owned() + "/projects.json").unwrap()).unwrap()["dir"].to_string()
}

fn get_term() -> String
{
    json::parse(&fs::read_to_string(dirs::config_dir().unwrap().to_str().unwrap().to_owned() + "/projects.json").unwrap()).unwrap()["term"].to_string()
}

fn search (param: &str) -> Vec<String>
{
    let mut haystack = vec![];

    for file in fs::read_dir(get_dir()).unwrap() {
        let path = file.unwrap().path();
        let s: String = path.to_str().unwrap().to_owned();
        if s.contains(param) {
            haystack.push(s);
        }
    }

    haystack

}

fn wr (needle: &str, haystack: Vec<String>, j: &i32)
{

    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    _ = write!(stdout, "{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), needle);
    
    let mut i: u16 = 0;
    
    for hay in haystack {
        _ = write!(stdout, "{}", termion::cursor::Goto(1, i+2));
        if i32::from(i) != j.to_owned() {
            _ = write!(stdout, "{}{}{}", color::Fg(color::Reset), color::Bg(color::Reset), hay);
        } else {
            _ = write!(stdout, "{}{}{}", color::Bg(color::Blue), color::Fg(color::Black), hay);
        }
        i += 1;
    }    
    
    _ = write!(stdout, "{}", termion::cursor::Goto((needle.len() as u16) + 1, 1));

    _ = stdout.flush();
}

fn main() 
{

    let args: Vec<String> = env::args().collect();

    let mut _stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let mut res = search("");

    wr("", res, &0);

    let mut needle: String = "".to_owned();

    let stdin = stdin();

    let mut j = 0;

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Down) => { 
                j += 1;
                res = search(&needle);
                wr(&needle, res, &j);
            },
            Event::Key(Key::Up) => { 
                j -= 1;
                res = search(&needle);
                wr(&needle, res, &j);
            },
            Event::Key(Key::Esc) => { return; },
            Event::Key(Key::Backspace) => { 
                _ = needle.pop(); 
                res = search(&needle);
                wr(&needle, res, &j);
            },
            Event::Key(Key::Char('\n')) => { break; },
            Event::Key(Key::Char(x)) => { 
                needle.push(x); 
                res = search(&needle);
                wr(&needle, res, &j);
            },
            _ => { }
        }
    }

    res = search(&needle);
   
    _ = Command::new("sh").arg("-c").arg("cd ".to_owned() + &res[j as usize] + "; " + &get_term()).spawn();
    _ = Command::new("kill").arg(&args[1 as usize]).spawn();

}
