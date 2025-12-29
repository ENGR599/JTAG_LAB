use std::collections::HashMap;

use eyre::{Result, eyre};

use crate::{
    io::{
        backend::{Backend, Data},
        devices::{DeviceInfo, IdCode},
    },
    jtag::{PATHS, Path, State},
    units::{Bits, Bytes},
};

pub struct Controller<B> {
    backend: B,
    info: DeviceInfo,
    buf: Vec<u8>,
}

fn detect_chain<B: Backend>(
    backend: &mut B,
    devices: &HashMap<IdCode, DeviceInfo>,
) -> Result<Vec<(u32, DeviceInfo)>> {
    let buf = &mut Vec::new();
    let reset_to_idle = PATHS[State::TestLogicReset][State::RunTestIdle];
    backend.tms(buf, Path::RESET)?;
    backend.tms(buf, reset_to_idle)?;

    let idle_to_sdr = Some(PATHS[State::RunTestIdle][State::ShiftDR]);
    let sdr_to_idle = Some(PATHS[State::ShiftDR][State::RunTestIdle]);

    let get_info = |idcode| -> Result<DeviceInfo> {
        let Some(info) = devices.get(&IdCode::new(idcode)) else {
            return Err(eyre!("idcode {idcode:08X} not found in device list"));
        };
        assert!(info.irlen <= Bits(32));
        Ok(info.clone())
    };

    let mut ret = Vec::new();
    loop {
        let to_read = Bytes((ret.len() + 1) * 4);
        backend.bytes(buf, idle_to_sdr, Data::Rx(to_read), sdr_to_idle)?;
        backend.flush(buf)?;

        let (ids, []) = buf.as_chunks() else {
            return Err(eyre!(
                "failed to fill idcode, or returned extra data: {buf:02X?}"
            ));
        };
        let id = ids[ret.len()];
        match u32::from_le_bytes(id) {
            // reached end of chain
            0xffff_ffff => {
                break;
            }

            // IDCODE guaranteed to start with a `1` bit, BYPASS as a single `0`. Nothing we can
            // really do with a device in BYPASS. Could scan a pattern through IR until we see our
            // input fed back, but that's annoying. All devices we care about start with IDCODE
            // anyway.
            idcode if idcode & 1 != 1 => {
                return Err(eyre!("device in BYPASS detected: {idcode:08X}",));
            }

            idcode => {
                let info = get_info(idcode)?;
                ret.push((idcode, info.clone()));
            }
        }
        buf.clear();
    }

    Ok(ret)
}

impl<B: Backend> Controller<B> {
    pub fn new(mut backend: B, devices: &HashMap<IdCode, DeviceInfo>) -> Result<Self> {
        let (_idcode, info) = match &detect_chain(&mut backend, devices)?[..] {
            [single] => single.clone(),
            [] => {
                return Err(eyre!("no devices detected on jtag chain"));
            }
            multiple => {
                let idcodes = multiple
                    .iter()
                    .map(|(idcode, _)| idcode)
                    .collect::<Vec<_>>();
                return Err(eyre!(
                    "multiple devices detected on jtag chain: {idcodes:08X?}"
                ));
            }
        };
        Ok(Self {
            backend,
            buf: Vec::new(),
            info,
        })
    }

    /// Run a set of commands, returning the data read out of TDO.
    ///
    /// Before the first command is run, the JTAG will be in
    /// [`State::RunTestIdle`].
    ///
    /// When IO occurs, the number of bytes read is sent over `sender`.
    pub fn run<'d>(&mut self, commands: impl IntoIterator<Item = Command<'d>>) -> Result<&[u8]> {
        let Self { backend, buf, info } = self;
        buf.clear();

        let ir0 = Some(PATHS[State::RunTestIdle][State::ShiftIR]);
        let ir1 = Some(PATHS[State::ShiftIR][State::RunTestIdle]);
        let dr0 = Some(PATHS[State::RunTestIdle][State::ShiftDR]);
        let dr1 = Some(PATHS[State::ShiftDR][State::RunTestIdle]);

        for command in commands {
            match command {
                Command::IrTxBits { tdi } => backend.bits(buf, ir0, tdi, info.irlen, ir1)?,
                Command::DrTx { tdi } => backend.bytes(buf, dr0, Data::Tx(tdi), dr1)?,
                Command::DrRx { len } => backend.bytes(buf, dr0, Data::Rx(len), dr1)?,
                Command::DrTxRx { tdi } => backend.bytes(buf, dr0, Data::TxRx(tdi), dr1)?,
                Command::Idle { len } => {
                    backend.bytes(buf, None, Data::ConstantTx(false, len), None)?
                }
            }
        }

        backend.tms(buf, Path::IDLE)?;
        backend.flush(buf)?;
        Ok(&self.buf)
    }
}

// weird public outer struct + priveate inner enum to only allow construction
// using the functions
//
// nothing _technically_ wrong with using the enum variants directly, but it
// clutters up tab completion and might be confusing
#[derive(Clone, Copy, Debug)]
pub enum Command<'d> {
    IrTxBits { tdi: u32 },

    DrTx { tdi: &'d [u8] },
    DrRx { len: Bytes<usize> },
    DrTxRx { tdi: &'d [u8] },

    Idle { len: Bytes<usize> },
}

#[allow(unused)]
impl<'d> Command<'d> {
    pub fn ir(tdi: u32) -> Self {
        Self::IrTxBits { tdi }
    }

    pub fn dr_tx(tdi: &'d [u8]) -> Self {
        Self::DrTx { tdi }
    }

    pub fn dr_rx(len: Bytes<usize>) -> Self {
        Self::DrRx { len }
    }

    pub fn dr_txrx(tdi: &'d [u8]) -> Self {
        Self::DrTxRx { tdi }
    }

    pub fn idle(len: Bytes<usize>) -> Self {
        Self::Idle { len }
    }
}
