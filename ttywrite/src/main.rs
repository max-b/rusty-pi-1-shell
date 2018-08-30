extern crate serial;
extern crate structopt;
extern crate xmodem;
#[macro_use] extern crate structopt_derive;

use std::path::PathBuf;
use std::time::Duration;

use structopt::StructOpt;
use serial::SerialPort;
use serial::core::{CharSize, BaudRate, StopBits, FlowControl, SerialDevice};
use xmodem::{Xmodem, Progress};

mod parsers;

use parsers::{parse_width, parse_stop_bits, parse_flow_control, parse_baud_rate};

#[derive(StructOpt, Debug)]
#[structopt(about = "Write to TTY using the XMODEM protocol by default.")]
struct Opt {
    #[structopt(short = "i", help = "Input file (defaults to stdin if not set)", parse(from_os_str))]
    input: Option<PathBuf>,

    #[structopt(short = "b", long = "baud", parse(try_from_str = "parse_baud_rate"),
                help = "Set baud rate", default_value = "115200")]
    baud_rate: BaudRate,

    #[structopt(short = "t", long = "timeout", parse(try_from_str),
                help = "Set timeout in seconds", default_value = "10")]
    timeout: u64,

    #[structopt(short = "w", long = "width", parse(try_from_str = "parse_width"),
                help = "Set data character width in bits", default_value = "8")]
    char_width: CharSize,

    #[structopt(help = "Path to TTY device", parse(from_os_str))]
    tty_path: PathBuf,

    #[structopt(short = "f", long = "flow-control", parse(try_from_str = "parse_flow_control"),
                help = "Enable flow control ('hardware' or 'software')", default_value = "none")]
    flow_control: FlowControl,

    #[structopt(short = "s", long = "stop-bits", parse(try_from_str = "parse_stop_bits"),
                help = "Set number of stop bits", default_value = "1")]
    stop_bits: StopBits,

    #[structopt(short = "r", long = "raw", help = "Disable XMODEM")]
    raw: bool,
}

fn progress_fn(progress: Progress) {
    println!("Progress: {:?}", progress);
}

fn send_data<T, U> (in_stream: &mut T, out_stream: &mut U, raw: bool) -> Result<(u64), std::io::Error> 
    where T: std::io::Read + std::fmt::Debug, U: std::io::Write + std::io::Read
{
    if raw {
        std::io::copy(in_stream, out_stream)
    } else {
        match Xmodem::transmit_with_progress(in_stream, out_stream, progress_fn) {
            Ok(num_bytes) => Ok(num_bytes as u64),
            Err(err) => Err(err)
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    use std::fs::File;
    use std::io::{self, BufReader};

    let opt = Opt::from_args();
    let mut port = serial::open(&opt.tty_path).expect("path points to invalid TTY");

    port.reconfigure(&|settings| {
        settings.set_baud_rate(opt.baud_rate)?;
        settings.set_char_size(opt.char_width);
        settings.set_stop_bits(opt.stop_bits);
        settings.set_flow_control(opt.flow_control);
        Ok(())
    })?;

    SerialDevice::set_timeout(&mut port, Duration::new(opt.timeout, 0))?;

    let result = match opt.input {
        Some(path) => {
            let file = File::open(path)?;
            let mut buf_reader = BufReader::new(file);
            send_data(&mut buf_reader, &mut port, opt.raw)
        },
        None => {
            send_data(&mut io::stdin(), &mut port, opt.raw)
        }
    };

    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}
