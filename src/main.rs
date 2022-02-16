use std::fs::{File, OpenOptions};
use std::io::{self, stdin, stdout, Write};
use std::path::Path;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn write_file(time_elapsed: f64, method: &str) -> io::Result<()> {
    let filename: &str = "/mnt/c/Users/Ryan Kooy/Desktop/andthereitis.txt";

    if !Path::new(&filename).exists() {
        let _ = File::create(filename)?;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(filename)
        .unwrap();

    if let Err(e) = writeln!(file, "{:.2} ({})", time_elapsed, method) {
        eprintln!("Couldn't write to file: {}", e);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let mut sout = stdout().into_raw_mode().unwrap();
    write!(
        sout,
        "{}{}Ctrl + z: ZZZ, Ctrl + y: YAK, Ctrl + g: MIA, Ctrl + w: WEB{}",
        termion::cursor::Goto(1, 1),
        termion::clear::All,
        termion::cursor::Hide
    ).unwrap();
    sout.flush().unwrap();

    let mut method: &str = "";

    // detect keydown events
    for k in stdin().keys() {
        write!(
            sout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        ).unwrap();

        match k.unwrap() {
            Key::Ctrl('g') => {
                method = "mia";
                break;
            },
            Key::Ctrl('y') => {
                method = "yak";
                break;
            },
            Key::Ctrl('z') => {
                method = "zzz";
                break;
            },
            Key::Ctrl('w') => {
                method = "web";
                break;
            },
            _ => (),
        }

        sout.flush().unwrap();
    }

    let now = Instant::now();
    let waiting: Vec<&str> = ["*..", ".*.", "..*"].to_vec();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut sout = stdout().into_raw_mode().unwrap();

        loop {
            println!("{}", method);
            thread::sleep(Duration::from_millis(300));
            write!(sout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).unwrap();
            sout.flush().unwrap();

            for i in &waiting {
                println!("{}", i);
                thread::sleep(Duration::from_millis(100));
                write!(sout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).unwrap();
                sout.flush().unwrap();
            }

            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("Terminating");
                    break;
                }
                Err(TryRecvError::Empty) => ()
            }
        }
    });

    let _ = stdin().lock().read_line();
    let _ = tx.send(());

    let total: f64 = (now.elapsed().as_secs() as f64) / 60.0;
    println!("{:.2} min ({})", total, method);

    write_file(total, method)?;

    write!(sout, "{}{}", termion::cursor::Goto(1, 1), termion::cursor::Show).unwrap();

    Ok(())
}
