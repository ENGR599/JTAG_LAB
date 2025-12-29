#import "signals.typ": signals

#set page(width: auto, height: auto, margin: 0.5in)
#set text(font: "Maple Mono NF")

#let signals_(..args) = text(12pt, signals(text-size: 10pt, ..args))
#let jtag(tms, tdi, tdo) = box(grid(
  columns: 2,
  inset: 4pt,
  align: (right + horizon, left),
  [*tms*], tms,
  [*tdi*], tdi,
  [*tdo*], tdo,
))

= Read XADC Registers
#let s = (
  (("=.", "SIR"), "+.", "+."),
  ("0...", ("=...", "XADC_DRP"), "x..."),
  (("=.", "SDR"), "+.", "+."),
  ("0...", ("=...", "R0"), "x..."),
  ("0...", ("-...", "R1"), ("-...", "R0")),
  ("0...", ("=...", "R2"), ("=...", "R1")),
  ("0...", "+...", ("-...", "R2")),
)

#jtag(
  signals_(..s.map(((tms, tdi, tdo)) => tms)),
  signals_(..s.map(((tms, tdi, tdo)) => tdi)),
  signals_(..s.map(((tms, tdi, tdo)) => tdo)),
)
