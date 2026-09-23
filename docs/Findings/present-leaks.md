# present-leaks — Phase 3 adversary

Date: 19 September 2026. Adversary **fired**.

The roadmap's Phase 3 adversary is: a cell that cannot describe itself
without knowing what will draw it. I wrote a body that reads `columns`
(`corpus/phase3/columns_reader.body`) and asked `describe` whether it
needed that signal.

`describe` never consults `Signals`. A `Description` has cell hash,
instance, ports, values, names, labels and roles. There is no width,
no column count, no host field. The body that reads `columns` is
refused at `check_signals` when the host declares none, and that
refusal names `columns`. That is the environment mechanism (R14), not
a leak in `present`.

What `present` turned out to need: **nothing from the host.** The
upward wrap holds. Turning a description into text is `joinn-cli`'s
template substitution; the protocol value does not ask how wide the
terminal is.

R44 stays open: a renderer may still want extent. That is a Phase 6
question, not a Phase 3 leak.
