use eyre::Result;

use crate::util::ShortHex;

mod io;
mod jtag;
mod lab;
mod units;
mod util;

fn main() -> Result<()> {
    init_logging()?;

    let vid = 0x0403;
    let pid = 0x6010;
    let dev = ftdi::find_by_vid_pid(vid, pid).open()?;

    let clock_frequency = Some(10_000_000);
    let device = io::cables::ftdi::Device::new(dev, clock_frequency)?;

    let device_list = io::devices::builtin().collect();
    let mut controller = io::Controller::new(device, &device_list)?;

    println!(" idcode: {:08X}", lab::step_1(&mut controller)?);
    println!("    dna: {}", ShortHex(lab::step_2(&mut controller)?));
    lab::step_3(&mut controller)?;

    if let Some(xadc) = lab::step_4(&mut controller)? {
        println!("   temp: {}", xadc.temperature);
        println!("vcc_int: {}", xadc.vcc_int);
        println!("vcc_int: {}", xadc.vcc_aux);
    }

    Ok(())
}

fn init_logging() -> Result<()> {
    use tracing_subscriber::{EnvFilter, fmt, prelude::*};
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .with(tracing_error::ErrorLayer::default())
        .init();
    Ok(())
}
