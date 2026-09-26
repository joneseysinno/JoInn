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
| R59 | Interactive hosts and races | 5.2 | open | |
| R60 | Every file kind ships its mutants | 5.2 | decided | Option A (AJ, 26 Sep): new grammar ships its mutants |
| R61 | Whose budget is it | 5.2 | open | |
| R62 | One host, both sides | 5.2 | open | |
| R64 | Legacy gates upgrade | 5.2 | open | |
| R65 | Laws that span bodies | 4 | open | |
| R66 | Loops with many frame changes | 4 | open | |
| R67 | Hyperedges in the assay | 4 | open | |
| R68 | Promises and allele bodies | 4 | open | |
| P4-adv | decoration check | 4 | fired | Findings/decoration-check.md |

