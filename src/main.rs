use std::thread::sleep;
use std::{io, time};
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering, AtomicUsize};
use std::sync::Arc;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};


fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let count   = Arc::new(AtomicUsize::new(0));

    let r = Arc::clone(&running);
    let c = Arc::clone(&count);

    ctrlc::set_handler(move || {
        play_once().expect("meowrror");
        let n = c.fetch_add(1, Ordering::SeqCst) + 1;
        if n == 12 {
            r.store(false, Ordering::SeqCst);
        }
    })
    .expect("Error setting Ctrl-C handler");
    while running.load(Ordering::SeqCst) {
        loop {
            if !running.load(Ordering::Relaxed) {
            break;
        }
        print!("\r           ");
        print!("\rHello");
        io::stdout().flush().unwrap();
        sleep(time::Duration::from_millis(50));
        for i in 0..=3{
             if !running.load(Ordering::Relaxed){
                break;
            }
            print!("\rHello{}", ".".repeat(i));
            io::stdout().flush().unwrap();
            sleep(time::Duration::from_secs(1));
        }
    }
    }
    println!("\nPurr-sleep mode... remain calm, no need to claw anyone.");
    sleep(time::Duration::from_secs(5));
}

fn play_once() -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    let file = File::open("assets/meow.mp3").expect("path error");
    let source = Decoder::new(BufReader::new(file))?;
    sink.append(source);
    sink.sleep_until_end();
    println!("Mrrp! No meow-ruptions! 😾");
    Ok(())
}