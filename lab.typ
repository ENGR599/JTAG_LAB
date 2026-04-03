#set page(paper: "us-letter")
#set page(margin: (x: 1in, y: 0.95in))
#let size = 10pt
#set text(font: "Maple Mono NF", size)
#show raw: it => text(fill: orange, size, it.text)
#show link: text.with(fill: blue)
#title[= Lab 3: Fun with JTAG!]

= Requirements

(IU machines should have everything available)

- Rust toolchain (`cargo`, `rustc`, recommend `rust-analyzer`)
- Basys3 board + micro-usb cable.

= Getting Started

+ Clone this repository to your local machine.
+ Enter the root of the cloned directory and run `cargo build`.

In this lab we will be using the Rust programming language to speak JTAG to the
Artix-7 FPGA on the Basys3 boards.

You will ONLY need to edit the lab `step_X` functions found in `lab.rs`. The
functions you will be using are:
- Write to the JTAG Instruction Register.
  - `Command::ir(0bXXXXXXX)`
- Write and read from the Data Register
  - `Command::dr_rx(Bytes(N))` - Read back Data Register output.
  - `Command::dr_tx(&data)` - Send bytes into the Data Register
  - `Command::dr_txrx(&data)` - Send bytes into the Data Register and read the
    same amount of bytes from Data Register.
- Keep the device in the Idle state for a given number of clock cycles.
  - `Command::idle(Bytes(N))`

= Instructions

Implement functions in `src/lab.rs`. Each function you must implement is found
in `step_N`. In these functions there is a `todo!()` placeholder where your code
should go.

+ read `IDCODE`
+ read `FUSE_DNA`
+ program a bitstream
+ (optional, more difficult) read information from `XADC`

= Hints

You will *NEED* to find JTAG IR registers by reviewing the User Guides below.

Information for the optional XADC step is found in a separate guide.

/ UG470: 7 Series FPGAs Configuration \ #link("https://docs.amd.com/api/khub/documents/FOs3lXmlcWxBhTIFxVKyGA/content")
/ UG480: 7 Series FPGAs XADC \ #link("https://docs.amd.com/api/khub/maps/qOeib0vlzXa1isUAfuFzOQ/attachments/_mT0t4XmsgJ2qfoNRTv53w-qOeib0vlzXa1isUAfuFzOQ/content")
/ jtag commands: UG470, "FPGA Boundary-Scan Instructions"
/ bitstream programming: UG470, "Device Configuration Flow Diagram"
/ xadc: UG480, "XADC DRP JTAG Read Operation"
