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

