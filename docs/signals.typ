#let signals(text-size: 1em, ..input) = {
  let get-str(arg) = if type(arg) == str { arg } else { arg.at(0) }
  let get-msg(arg) = if type(arg) == str { none } else { arg.at(1) }

  let between = 0.2em
  let fudge = 0.4pt
  let get-between(prev, next) = {
    let x0 = -fudge
    let x1 = between + fudge

    if type(prev) != array and type(next) != array {
      // single -> single
      return box(width: between, line(
        start: (x0, prev),
        end: (x1, next),
      ))
    }

    let p = if type(prev) == array { prev } else { (prev, prev) }
    let n = if type(next) == array { next } else { (next, next) }

    let adjust(x) = if x < 0.5em { x - fudge } else { x + fudge }
    let no_adjust(x) = x
    let dy0 = if p.at(0) != n.at(0) { adjust } else { no_adjust }
    let dy1 = if p.at(1) != n.at(1) { adjust } else { no_adjust }

    box(
      width: between,
      height: 1em,
      curve(
        curve.move((x0, dy0(p.at(0)))),
        curve.line((x1, dy0(n.at(0)))),
        curve.move((x0, dy1(p.at(1)))),
        curve.line((x1, dy1(n.at(1)))),
      ),
    )
  }

  let get-line(c, prev, new_start) = if type(new_start) == array {
    let extra = if c == "x" or c == "+" {
      let start = if prev == c { -0.2em } else { 0em }
      let len = if prev == c { 6 } else { 5 }
      curve(
        stroke: (thickness: 0.5pt, cap: "round", paint: gray),
        ..range(0, len)
          .map(x => (
            curve.move((start + 0.2em * x, 0.9em)),
            curve.line((0.2em, -0.8em), relative: true),
          ))
          .flatten(),
      )
    } else { none }
    let curve = curve(
      curve.move((0em, new_start.at(0))),
      curve.line((1em, new_start.at(0))),
      curve.move((1em, new_start.at(1))),
      curve.line((0em, new_start.at(1))),
      curve.move((0em, new_start.at(0))),
    )
    (curve, extra)
  } else {
    (line(start: (0em, new_start), end: (1em, new_start)), none)
  }

  let start_ = (
    "0": 1em,
    "1": 0em,
    "=": (0em, 1em),
    "-": (1em, 0em),
    "x": (0em, 1em),
    "+": (1em, 0em),
  )
  let start = start_.at(get-str(input.at(0)).at(0))
  let prev = none
  let cells = ()
  let backgrounds = ()
  let texts = ()

  let idx = 0

  for arg in input.pos() {
    let idx-start = cells.len()

    for c in get-str(arg) {
      if c == " " { continue }
      if c == "." { c = prev }

      let new_start = start_.at(c)
      if idx != 0 {
        cells.push(get-between(start, new_start))
      }
      let (cell, background) = get-line(c, prev, new_start)
      cells.push(cell)
      if background != none {
        backgrounds.push(grid.cell(x: idx, y: 0, background))
      }

      prev = c
      start = new_start
      idx += 1
    }

    let idx-end = cells.len()
    let msg = get-msg(arg)
    if msg != none {
      let width = idx-end - idx-start
      texts.push(grid.cell(
        x: calc.floor((idx-start + 1) / 2),
        y: 0,
        colspan: calc.floor((idx-end - idx-start + 1) / 2),
        box(height: 1em, width: 100%, text(text-size, msg)),
      ))
    }
  }

  let ex-cols = calc.floor((cells.len() + 1) / 2) * (1em + between,)
  place(grid(columns: ex-cols, align: horizon, ..backgrounds))
  place(grid(columns: ex-cols, inset: (x: 0.2em), align: horizon, ..texts))
  grid(columns: (auto,) * cells.len(), ..cells)
}
