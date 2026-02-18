use std::{env, fs, io, process::Command, thread, time::Duration};
use tracing::{debug, error, info, level_filters::LevelFilter, warn};

fn main() {
    let level_filter: u8 = match env::var("TRACING_LEVEL_FILTER") {
        Ok(level_filter) => level_filter.parse::<u8>().unwrap_or_else(|_| 2u8),
        Err(_) => 2u8
    };

    let level_filter = match level_filter {
        0 => LevelFilter::OFF,
        1 => LevelFilter::ERROR,
        2 => LevelFilter::WARN,
        3 => LevelFilter::INFO,
        4 => LevelFilter::DEBUG,
        _ => LevelFilter::TRACE,
    };

    tracing_subscriber::fmt()
        .with_max_level(level_filter)
        .init();

    info!("level filter: {level_filter}");

    let port_name = match env::var("SERIAL_PORT_NAME") {
        Ok(port_name) => port_name,
        Err(e) => {
            warn!("SERIAL_PORT_NAME: {e}");
            "/dev/ttyUSB0".to_owned()
        }
    };
    info!("port: {port_name}");

    let baud_rate = match env::var("SERIAL_PORT_BAUD_RATE") {
        Ok(baud_rate_str) => baud_rate_str.parse().unwrap_or_else(|_| { warn!("Failed to parse baud rate as u32 from \'{baud_rate_str}\'"); 115200}),
        Err(e) => {
            warn!("SERIAL_PORT_BAUD_RATE: {e}");
            115200
        }
    };
    info!("baud rate: {baud_rate}");

    info!("current directory: {:?}", env::current_dir());

    loop {
        run(port_name.as_ref(), baud_rate);
        thread::sleep(Duration::from_secs(5));
    }
}

fn run(port_name: &str, baud_rate: u32) {
    let mut sp = match serialport::new(port_name, baud_rate).timeout(Duration::from_secs(1)).open() {
        Ok(sp) => {
            sp
        }
        Err(e) => {
            error!("Failed to open port with name \'{port_name}\'. Error :{e}");
            return;
        }
    };
    let mut buf: Vec<u8> = vec![0; 128];

    loop {
        match sp.read(&mut buf) {
            Ok(n) => {
                if n > 0 {
                    let received = String::from_utf8_lossy(&buf[..n]);
                    if received.starts_with("runfile") {
                        let mut iter = received.split_whitespace();
                        _ = iter.next();
                        match iter.next() {
                            Some(file_str) => {
                                match fs::metadata(file_str) {
                                    Ok(md) => {
                                        if !md.is_file() {
                                            error!("\'{file_str}\' is not a file");
                                            continue;
                                        }
                                        match Command::new(format!("./{file_str}")).output() {
                                            Ok(output) => debug!(
                                                "\'{}\' has exited with status: {}",
                                                file_str,
                                                output.status
                                            ),
                                            Err(e) => error!("failed to run script: {e}"),
                                        }
                                    },
                                    Err(e) => {
                                        match e.kind() {
                                            io::ErrorKind::NotFound => error!("NotFound. current directory: {:?}, file: {file_str}", env::current_dir()),
                                            io::ErrorKind::PermissionDenied => error!("PermissionDenied. current directory: {:?}, file: {file_str}", env::current_dir()),
                                            _ => error!("{e}")
                                        }
                                    },
                                }
                            },
                            None => warn!("incorrect format: {received}"),
                        }
                    } else {
                        warn!("unknown message: {}", received)
                    }
                }
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            Err(e) => {
                error!("Pipe closed. Error: {e}");
                break;
            }
        }
    }
}
