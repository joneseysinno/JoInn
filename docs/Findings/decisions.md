# Decision ledger

One line per D/V/R. Status is `holds`, `open`, or `reversed`. A `reversed`
line must name a file under `docs/Findings/`.

| id | statement | phase | status | later |
|---|---|---|---|---|
| D-desc | A description is a value, not text | 3 | holds | |
| D-intent | An intent is derived from the body | 3 | holds | |
| D-env | The environment is a body; hosts declare emitted signals | 3 | holds | |
| D-probe | probe is read-only and takes `&BodyState` | 3 | holds | |
| D-order | Transcript moves to joinn-cli before joinn-run is deleted | 3 | holds | |
| D-artifact | A control is an artifact, not a predicate | 3 | holds | |
| D-scan | A source check scans a tree, not a file | 3 | holds | |
| D-module | module.rs beside module/; never mod.rs | 3 | holds | |
| D-io | std::io and std::fs only in host crates and xtask | 3 | holds | |
| V59 | Description has a canonical form and a hash | 3 | holds | |
| V60 | Two hosts produce byte-identical descriptions | 3 | holds | |
| V61 | A regulatory edit moves a description, never a cell hash | 3 | holds | |
| V62 | A host may only emit an intent in intent_set | 3 | holds | |
| V63 | Reading an undeclared signal is refused naming it | 3 | holds | |
| V64 | probe writes nothing and draws no budget | 3 | holds | |
| V65 | No crate but a host crate and xtask names std::io or std::fs | 3 | holds | |
| V69 | The Phase 2 transcript is produced by the current host | 3 | holds | |
| R14 | Environment signals | 3 | open | extended |
| R39 | Counterfeit strength | 2.2 | open | Findings/counterfeit-strength.md |
| R41 | Testimony corpus | 2.2 | open | |
| R44 | What else is in a description | 3 | open | |
| R45 | Is role a closed enum | 3 | open | |
| R46 | Does a host owe a refusal a description | 3 | open | |
| R47 | Is the environment body gated | 3 | open | |
| R14-adv | present-leaks adversary | 3 | holds | Findings/present-leaks.md |
| R55 | augmented complex on a link | 5.1 | open | |
| R56 | embeddings across a link | 5.1 | open | |
| R57 | a universe that waits half-fed | 5.1 | open | |
| R58 | correlation of a reply with its question | 5.1 | open | |
| R63 | vocabulary ban against a host language | 5.2 | holds | JoInn's language only (§2.14) |
| V98 | A control never sees bytes; an artifact that doesn't parse refuses the run | 5.2 | holds | |
| V99 | Every catalogue mutant parses, moves the hash, and has a named downstream effect | 5.2 | holds | |
| V100 | Each non-legacy control flips on its declared mutant | 5.2 | holds | |
| V101 | Each non-legacy control ignores its neutral edit | 5.2 | holds | |
| V102 | The harness fixtures come out right before any gate runs | 5.2 | holds | |
| V103 | No two non-legacy items share (artifact, opposes) | 5.2 | holds | |
| V104 | A Bound can only come from a store | 5.2 | holds | |
| V105 | A hash the store doesn't hold is refused naming the alias | 5.2 | holds | |
| V106 | A body refusal is a report, and the universe keeps running | 5.2 | holds | |
| V107 | LinkRefusal::Refused only for a delivery in the same pass | 5.2 | holds | |
| V108 | Grants are declared; no host calls grant | 5.2 | holds | |
| V109 | A host's output doesn't depend on link ids or aliases | 5.2 | holds | |
| V110 | The far side is a kind at both hosts; the probe has the words | 5.2 | holds | |
| V111 | One coding hash, one face; results don't depend on folder order or empty folders | 5.2 | holds | |
| V112 | Ranks are exact over ℚ, and b₀ − b₁ + b₂ = V − E + F on every complex, or the assay refuses | 4 | holds | |
| V113 | The fast and reference derivations agree on every corpus subject; an injected disagreement is a truth violation | 4 | holds | |
| V114 | An assay report does not move under a neutral edit, an allele strip, a lens drop, or a rename (apart from the renamed names) | 4 | holds | |
| V115 | An assay never refuses on its own; only a declaration refuses | 4 | holds | |
| V116 | Every filling names the law or frame that made it; nothing fills a loop except §2.3's three rules | 4 | holds | |
| V117 | New grammar ships its mutants before a control points at it (R60) | 4 | holds | |
| V118 | `joinn-assay` depends on `joinn-frame` alone and names no DNA type | 4 | holds | |
| R59 | Interactive hosts and races | 5.2 | open | |
| R60 | Every file kind ships its mutants | 5.2 | decided | Option A (AJ, 26 Sep): new grammar ships its mutants |
| R61 | Whose budget is it | 5.2 | open | |
| R62 | One host, both sides | 5.2 | open | |
| R64 | Legacy gates upgrade | 5.2 | open | |
| R65 | Laws that span bodies | 4 | open | |
| R66 | Loops with many frame changes | 4 | open | |
| R67 | Hyperedges in the assay | 4 | open | |
| R68 | Promises and allele bodies | 4 | open | |
| R69 | Descriptions of an instance that fires twice in a round | 4 | open | |
| P4-adv | decoration check | 4 | held | Findings/decoration-check.md |
| R70 | A real weak device | 6 | open | |
| R71 | Authored layout | 6 | open | |
| R72 | The vertex-buffer fallback | 6 | open | |
| R73 | Text on screen | 6 | open | |
| R74 | One address, one port | 6 | open | |
| R75 | Membrane ports | 6 | open | |
| VH1 | No crate above the renderer boundary names wgpu or winit | 6 | holds | Part II V1 |
| VH2 | GPU state discarded and regrown draws identical color and ID bytes | 6 | holds | Part II V2 |
| VH5 | Every non-edge pixel has one owner, and both pickers name it | 6 | holds | Part II V5 |
| VH11 | The device opens at WebGPU core limits and the bind-group layout fits | 6 | holds | Part II V11 |
| VH12 | An idle scene has nothing to draw; the shell redraws only on change | 6 | holds | Part II V12 |
| VH14 | A picker disagreement is a truth violation, on every adapter | 6 | holds | Part II V14 |
| V119 | A layout is blind to the regulatory region and to alleles | 6 | holds | |
| V120 | The grid picker and the brute-force picker agree on every pixel | 6 | holds | |
| V121 | One run writes at most Σ(1 + ports) over the instances it touched, plus one when it clears a refused bit it did not touch; a camera change writes none | 6 | holds | |
| V122 | Delta-built tables equal regrow, byte for byte | 6 | holds | |
| V123 | A stale pick is refused naming both generations | 6 | holds | |
| V124 | Floats exist only in joinn-gpu and joinn-shell-desktop | 6 | holds | |
| R76 | Order inside a body | V | decided | AJ 29 Sep: combine is order-free; − and ÷ are combine at a turn; presentation is the host's; other order is a system matter |
| R77 | Where a force's truth lives | V | decided | Part V C10, C16 |
| R78 | The Floor re-read as forces | V | attempted | Findings/the-floor.md §5, 29 Sep: no pair left; eq/choose kept paired (AJ) |
| R79 | Link policy: pinned, live, watch | V | open | Part V §7.2 |
| R80 | Wrapping a body | V | decided | Part V C17 |
| R81 | Roles of cells in a body | V | decided | AJ 29 Sep: (faces out/in) × (holds/reacts) = protect, carry, store, respond; derived, never written |
| R82 | The opposite of a lossy combine | 7 | open | |
| R83 | Multiplication | 7 | decided | AJ 30 Sep: multiply is combine across dimensions; Part VI K9–K10 (stacking, order-signed) |
| R84 | Many members, chained forces | 7 | open | |
| R85 | One port, two forces | 7 | open | |
| R86 | The two open Floor reductions | 7 | open | |
| V125 | The shell's own run path keeps V122: tables equal regrow after every Enter | 7 | holds | wired rows 2, 3, 4; contact rows 2, 1, 3 |
| V126 | A `.contact` file holds no wire: the parser refuses a wires section | 7 | holds | gate 7 item 1 |
| V127 | Member order is never hashed | 7 | holds | four spellings share 868e79b2… (gate 7 item 2) |
| V128 | `lower` is a function of the contact, its cells and the register only, and `contact_surface(c) = surface(lower(c))` | 7 | holds | gate 7 item 1 |
| V129 | Every registered combine is order-free and opposed by a registered separate (a turn of its response) | 7 | holds | `cargo xtask forces`; gate 7 item 2 |
| V130 | A force owns no pixel; a contact picture has no link owner | 7 | holds | gate 7 item 3 |
| V131 | Roles are derived and blind to the regulatory region | 7 | holds | |
| V132 | Device loss is reported by wgpu's callback, and the shell regrows from it | 7 | holds | |
| V133 | A contact body's forces form no loop: no force's response reaches, through members, back to itself | 7 | holds | Amendment B |
| R87 | Presentation names parts | 7.1 | open | |
| R88 | Base kinds | 7.1 | answered | Part VI K18–K19: no base kinds; counting, where, side |
| R89 | Orientation | 7.1 | answered | Part VI K13a: part of where; a sign convention is presentation |
| R90 | Is carry the mirror? | 7.1 | open | |
| R91 | Time as pieces | 7.1 | open | |
| R92 | The continuum | 7.1 | partly answered | Part VI K20: polynomial loads derive exactly |
| R93 | Beyond linear bridges | 7.1 | open | |
| R94 | Tags and the gate | 7.1 | open | |
| R95 | Sensitive problems | 7.1 | open | |
| R96 | Kinds as bodies | 7.1 | open | murky; go forward |
| R97 | Witness libraries | 7.1 | open | |
| R98 | Indeterminate beams | 7.1 | open | |
| R99 | Where a section property lives | 7.1 | open | |
| R100 | The decimal | 7.1 | open | |
| V134 | `present` refuses a port the scene does not hold, unless it is a contact's interior port | 7.1 | holds | P71-02: `present: port sum@9 is not in this scene; acceptance is a port of body body`; gate 6 and 7 3/3 |
| V135 | Balance and moment read no bridge; deflection reads exactly the bridge edition | 7.1 | holds | spike S8: bridge reads `[(0, 4), (0, 4), (0, 4), (0, 2), (0, 6)]`; the no-bridge plant refused ([findings](phase-7.1-beam-examples.md)) |
| V136 | Every derived value equals every witness that states it, exactly | 7.1 | holds | spike S8: `12 witness(es) agree, 0 disagree`; the wL²/12 plant refused |
| V137 | Refining the complex changes no derived value at any shared point | 7.1 | holds | spike S8: `refinement equal in 5`; the rectangle plant refused |
| V138 | `add` refuses unequal tags, even when their units print the same | 7.1 | holds | spike S8: the moment + work plant, `though both are kip·in` |
| V139 | Every moment is taken in one order; mixed order fails the closure check | 7.1 | holds | spike S8: `M at end 0` in all five; the mixed-order plant `M at end 1728/5 kip·ft, want 0` |
| V140 | Cursor sends no input outside a window it started (rule 67) | 7.1 | holds | rule 67; no step of Phase 7.1 opened a window or sent input |
| R101 | Where the cut lives | 7.2 | open | |
| R102 | Hysteresis or fade | 7.2 | open | |
| R103 | Text as cells | 7.2 | open | |
| R104 | Layout of a lens | 7.2 | open | |
| V141 | No float decides anything: bands, owners, the cut and the rebase are integer decisions, the same on CPU and GPU | 7.2 | holds | gate 7.2 item 2: disagree 0 at every §2.12 view and one fade-window view per threshold, on all three adapters; the shader's tests are `u32`/`i32`. No grove body sits at `10·s = 11·T` exactly, so a `>` for `≥` plant in the shader is not caught ([findings](phase-7.2-zoom.md)) |
| V142 | A pan or zoom writes the tick uniform and no row; a rebase writes chart rows only | 7.2 | holds | gate 7.2 item 1: 174 script steps, rows 0 except the rebase; the plant that writes chart rows on a plain zoom fails item 1 (`a pan or zoom at b0000 wrote 3209 row(s); acceptance is 0 (V142)`) |
| V143 | A rebase moves nothing: color and ID bytes identical before and after | 7.2 | holds | `rebase universe -> b0000 …: color identical, ids identical` on all three adapters (P72-10, gate 7.2 item 1) |
| V144 | Zoom about one pixel is reversible: n notches in and n out return the identical camera and bytes | 7.2 | holds | gate 7.2 item 1: 8 in and 8 out about (700, 300) return the camera, and the bytes on every adapter |
| V145 | Two cuts, one truth: the owners in the GPU image are exactly those the CPU cut allows | 7.2 | holds | `cargo xtask pick`: `owners <n> (cut allows <n>)` equal at every grove view on every adapter; gate 7.2 item 2 |
| V146 | A link touches each cut node at most once | 7.2 | holds | `cargo xtask zoom`: touches 264 at level −4 step 0 and 1182 framed, each node at most once per link; gate 7.2 item 3 |
| V147 | An idle window draws nothing, at any zoom (V12 extended) | 7.2 | holds | the shell session test: after the script `take_tick` is `None` twice. The window itself was not observed from Cursor (rule 67); AJ's window check is the window evidence |
| R105 | Snapshots | 7.3 | open | |
| R106 | Streets as structure | 7.3 | open | |
| R107 | The knot as a place | 7.3 | open | |
| R108 | The engine's cost per fire | 7.3 | open | |
| V148 | A hyperedge touches; it never crosses: §2.6's four checks hold for every route in every fold state | 7.3 | holds | `cargo xtask links`: `crossings 0` for universe.universe, ordered.universe and the grove in every fold state; gate 7.3 item 1: `body pixels a link owns 0` at every §2.12 view on all three adapters; drawing links over bodies fails it (4203 pixels) ([findings](phase-7.3-links.md)) |
| V149 | Routes live in gutters and are grown once per fold state; a zoom writes no row | 7.3 | holds | `cargo xtask regrow`: `routes 0 regrow` at every grove step, rows 0 except the rebase; the plant that rewrites routes on a same-anchor pan fails (`a pan or zoom at b0000 wrote 4779 row(s)`, P73-09) |
| V150 | Order is drawn only when declared: no spine and no arrowhead along an unordered link | 7.3 | holds | gate 7.3 item 2: grove `ordered 0, mid-path arrowheads 0, spines at 15 views 0`; ordered.universe's `path` a spine at its frame and at level −4; a mid-path arrowhead on unordered links fails it (146) |
| V151 | A folded node is touched once per link: one touch point, one leg | 7.3 | holds | gate 7.3 item 3: `systems folded 264 touches, 136 legs; open 1182 stubs`, each node touched once per link and every member stood for |
| V152 | Two pickers name every link pixel: CPU and GPU agree on link owners at every view | 7.3 | holds | `cargo xtask pick`: disagree 0 and `link owners n (cut allows n)` equal at every grove view and ordered.universe's galaxy, all three adapters (P73-08); gate 7.3 item 3: a leg pick `link sys_g0s00 member 3`, GPU equal on every adapter |
| G1 | The running example is a child's counting app with three systems: counting, UI, saves | Part VII | decided | |
| G2 | Adding addition adds to the counting system; it doesn't make a new one | Part VII | decided | |
| G3 | New systems are warranted as things progress, when the existing ones can't do what is needed | Part VII | decided | |
| G4 | The calculator is a system. A system holds forces, and may hold several | Part VII | decided | |
| G5 | A system is the bodies its forces reach | Part VII | decided | |
| G6 | A force is a directed hyperedge: it surrounds what it acts on and points to what it produces | Part VII | decided | |
| G7 | A force is drawn as a lasso hugging its tails, cinched to an arrow at its head | Part VII | decided | |
| G8 | A system starts as a seed; its DNA controls how its bodies grow when inputs are put in | Part VII | decided | |
| G9 | Two cells is the adding seed's starting shape, not a law; every size, 0 included, is true | Part VII | decided | |
| G10 | A body can grow to as many cells as it needs | Part VII | decided | |
| G11 | Growth is a fourth change, beside edit, edition and evolution | Part VII | decided | |
| G12 | Identity follows the DNA and forces; grown cells are state | Part VII | decided | |
| G13 | The basic math operations are all ways of counting, so they stay in one system | Part VII | decided | |
| G14 | Counting is the witness: every answer must agree with the counting it stands for | Part VII | decided | |
| G15 | The response is a cell at the lasso's point, and the inputs stay | Part VII | decided | |
| G16 | A new system is warranted exactly when what is needed can't be derived from an existing one | Part VII | decided | |
| G17 | A lens is the view | Part VII | decided | wording from the 7.4 plan; Part VII Draft 0.2 on disk lists G1–G16 |
| G18 | Links transfer data while forces stimulate | Part VII | decided | wording from the 7.4 plan; Part VII Draft 0.2 on disk lists G1–G16 |
| G19 | Things must be visually clear: a link and a force never look alike | Part VII | decided | wording from the 7.4 plan; Part VII Draft 0.2 on disk lists G1–G16 |
| R109 | Systems and lenses | 7.4 | answered | G17 |
| R110 | Links as carry | 7.4 | answered | G18 |
| R110a | Carry, the force | 7.4 | open | |
| R111 | What is consumed | 7.4 | open | |
| R112 | Counting and measuring | 7.4 | open | |
| R113 | The app as a cell | 7.4 | open | |
| R114 | Lineage as learning | 7.4 | open | |
| R115 | Removing a number | 7.4 | open | |
| R116 | Where state lives | 7.4 | open | |
| R117 | Systems of several bodies | 7.4 | open | |
| V160 | A body grows by its DNA, and every size is true: identity at 0, the input at 1, the combine at n | 7.4 | holds | P74-06 (76f3328); `cargo xtask grow`: §3's ten transcripts, `count 0` at size 0 and every count as predicted; gate 7.4 item 1, whose control flips when counting accepts the 3 (demo b) |
| V161 | Counting is the witness: after every growth step the response equals `count_witness` | 7.4 | holds | P74-07 (0df0880); `grow` prints the witness beside the engine at every step; its plant (a fold that drops its last member) is refused at n 2; gate 7.4 item 1: `witness agrees` on every transcript |
| V162 | Growth is not evolution: the system's hash is unchanged by growth; grown state is never hashed or stored | 7.4 | holds | P74-06 (76f3328); `grow`: `hash c211e65544e5` / `9bd87572c3bc` at every step, and a hash over the grown state is refused at n 1; gate 7.4 item 3: the hash at sizes 0 … 13 |
| V163 | Evolution keeps every old witness and gains an ability | 7.4 | holds | P74-07 (0df0880); gate 7.4 item 2: `evolution counting → adding: 3 witnesses hold; adding gains 3`, and adding made to accept one is refused as an edit; removing the gain test fails it (demo c) |
| V164 | A force is seen as a lasso: it surrounds what it reaches, never meets a cell, and points to its response; CPU and GPU name its pixels alike | 7.4 | holds | P74-09 (3547079), P74-11 (0596f31); `pick`: `disagree 0`, `force owners 1 (cut allows 1)` at sizes 0, 3, 7 of both systems on all three adapters; gate 7.4 item 3: rules at sizes 0 … 13, no body pixel owned by the force; the lasso drawn after the cells fails it (demo d) |
