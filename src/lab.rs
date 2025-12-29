#![allow(unreachable_code)]

use eyre::Result;

#[allow(unused)]
use crate::{
    io::{Backend, Command, Controller},
    units::Bytes,
};

/// Read the JTAG IDCODE
pub fn step_1<B: Backend>(controller: &mut Controller<B>) -> Result<u32> {
    let id: &[u8] = controller.run([
        // fill in commands here
        todo!(),
    ])?;
    let id: &[u8; 4] = id.try_into()?;
    Ok(u32::from_le_bytes(*id))
}

/// Read the FUSE DNA
pub fn step_2<B: Backend>(controller: &mut Controller<B>) -> Result<&[u8; 8]> {
    let fuse_dna: &[u8] = controller.run([
        // fill in commands here
        todo!(),
    ])?;
    Ok(fuse_dna.try_into()?)
}

/// Program a bitstream
pub fn step_3<B: Backend>(controller: &mut Controller<B>) -> Result<()> {
    const BITSTREAM: &[u8] = include_bytes!("../resources/top.bit");
    let mut bitstream_reversed = BITSTREAM.to_vec();
    for d in &mut bitstream_reversed {
        *d = d.reverse_bits();
    }

    controller.run([
        // fill in commands here
        todo!(),
    ])?;
    Ok(())
}

pub struct XadcInfo {
    pub temperature: f32,
    pub vcc_int: f32,
    pub vcc_aux: f32,
}

/// Read information from the XADC (optional)
pub fn step_4<B: Backend>(_controller: &mut Controller<B>) -> Result<Option<XadcInfo>> {
    Ok(None)
}
