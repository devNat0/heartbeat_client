use anyhow::Result;
use crossbeam_channel::{bounded, select, tick, Receiver};
use std::time::Duration;

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}

fn main() -> Result<()> {
    let device = std::env::args().nth(1).expect("no serial device specified");
    println!("Using:{}", device);
    let mut port = serialport::new(device, 9600)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    let ctrl_c_events = ctrl_channel()?;
    let ticks = tick(Duration::from_secs(1));

    loop {
        select! {
            recv(ticks) -> _ => {
                port.write(b"beat\0").expect("Write failed");
                println!("working!");
            }
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("Goodbye!");
                port.write(b"pause\0").expect("Write failed");
                break;
            }
        }
    }

    Ok(())
}
