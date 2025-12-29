use crate::units::Bits;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IdCode(u32);

#[derive(Clone, Debug)]
pub struct DeviceInfo {
    pub irlen: Bits<u8>,
    #[allow(unused)]
    pub name: &'static str,
}

impl IdCode {
    pub const fn new(code: u32) -> Self {
        /// IEEE 11491-2013, Figure 12-1, "Structure of the device
        /// identification code"
        const VERSION: u32 = 0xf0000000;
        Self(code & !VERSION)
    }

    pub const fn code(self) -> u32 {
        self.0
    }
}

impl From<u32> for IdCode {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<IdCode> for u32 {
    fn from(val: IdCode) -> Self {
        val.code()
    }
}

/// Returns iterator of `(idcode, info)`. Intended to be collected into a
/// `HashMap`, to be passed to [`crate::Controller::new`].
pub fn builtin() -> impl Iterator<Item = (IdCode, DeviceInfo)> {
    xilinx()
}

fn xilinx() -> impl Iterator<Item = (IdCode, DeviceInfo)> {
    use Bits as B;

    const fn id(code: u32) -> IdCode {
        IdCode::new(code)
    }

    // intentionally stripped-down metadata, only expect to use this with a basys 3
    #[rustfmt::skip]
    static DEVICES: &[(IdCode, DeviceInfo)] = &[
        // Spartan-7
        (id(0x03622093), DeviceInfo { irlen: B( 6), name: "XC7S6"     }),
        (id(0x03620093), DeviceInfo { irlen: B( 6), name: "XC7S15"    }),
        (id(0x037C4093), DeviceInfo { irlen: B( 6), name: "XC7S25"    }),
        (id(0x0362F093), DeviceInfo { irlen: B( 6), name: "XC7S50"    }),
        (id(0x037C8093), DeviceInfo { irlen: B( 6), name: "XC7S75"    }),
        (id(0x037C7093), DeviceInfo { irlen: B( 6), name: "XC7S100"   }),
        // Artix-7
        (id(0x037C3093), DeviceInfo { irlen: B( 6), name: "XC7A12T"   }),
        (id(0x0362E093), DeviceInfo { irlen: B( 6), name: "XC7A15T"   }),
        (id(0x037C2093), DeviceInfo { irlen: B( 6), name: "XC7A25T"   }),
        (id(0x0362D093), DeviceInfo { irlen: B( 6), name: "XC7A35T"   }),
        (id(0x0362C093), DeviceInfo { irlen: B( 6), name: "XC7A50T"   }),
        (id(0x03632093), DeviceInfo { irlen: B( 6), name: "XC7A75T"   }),
        (id(0x03631093), DeviceInfo { irlen: B( 6), name: "XC7A100T"  }),
        (id(0x03636093), DeviceInfo { irlen: B( 6), name: "XC7A200T"  }),
        // Kintex-7
        (id(0x03647093), DeviceInfo { irlen: B( 6), name: "XC7K70T"   }),
        (id(0x0364C093), DeviceInfo { irlen: B( 6), name: "XC7K160T"  }),
        (id(0x03651093), DeviceInfo { irlen: B( 6), name: "XC7K325T"  }),
        (id(0x03747093), DeviceInfo { irlen: B( 6), name: "XC7K355T"  }),
        (id(0x03656093), DeviceInfo { irlen: B( 6), name: "XC7K410T"  }),
        (id(0x03752093), DeviceInfo { irlen: B( 6), name: "XC7K420T"  }),
        (id(0x03751093), DeviceInfo { irlen: B( 6), name: "XC7K480T"  }),
        // Virtex-7
        (id(0x03671093), DeviceInfo { irlen: B( 6), name: "XC7V585T"  }),
        (id(0x03667093), DeviceInfo { irlen: B( 6), name: "XC7VX330T" }),
        (id(0x03682093), DeviceInfo { irlen: B( 6), name: "XC7VX415T" }),
        (id(0x03687093), DeviceInfo { irlen: B( 6), name: "XC7VX485T" }),
        (id(0x03692093), DeviceInfo { irlen: B( 6), name: "XC7VX550T" }),
        (id(0x03691093), DeviceInfo { irlen: B( 6), name: "XC7VX690T" }),
        (id(0x03696093), DeviceInfo { irlen: B( 6), name: "XC7VX980T" }),
    ];

    DEVICES.iter().cloned()
}
