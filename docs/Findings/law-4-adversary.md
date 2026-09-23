# Law 4 adversary · fired

Date: 22 September 2026. Hand-typed after P5-22.

The roadmap's Phase 5 adversary: *the container communication methods turn
out to want to be different per app.*

What the third body wanted: to hear both the calculator's sum and the units
scale in one place — a small `bus` body with a single membrane in-port
(`listen@0`), without inventing a fourth container method.

What was tried: a three-member hyperedge

```
link bus order none {
  calc.sum@2 tail
  units.scale@1 head
  bus.listen@0 head
}
```

plus ordinary wires inside each body for anything local.

**Law 4 holds.** The want is expressible with a wire (inside each body) and a
hyperedge (between the three bodies). No new container was required. The
adversary is marked **fired**; the finding is that Law 4 was enough.

`corpus/phase5/bus.body` is the body that asked. Its coding hash is
`fa812abd9ab0b1b6a3aef72b3c8ca1e2641c6f307350a8cc86ebea56187b8123`
(`joinn.body.v1`, hand-checked).
