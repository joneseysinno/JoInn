//! Canonical universe coding text, including the trailing newline.
//! allow(modules): prints link members inline beside the universe form

use std::fmt::Write as _;

use super::{Mark, Order, UniverseCoding};

/// Canonical universe coding text, including the trailing newline.
pub fn print_universe(coding: &UniverseCoding) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "universe {{");
    let _ = writeln!(out, "  codex {}", coding.codex);
    let _ = writeln!(out, "  bodies {{");
    let mut bodies = coding.bodies.clone();
    bodies.sort_by(|a, b| a.alias.cmp(&b.alias));
    for body in &bodies {
        let _ = writeln!(
            out,
            "    body:{} as {}",
            body.hash.to_hex(),
            body.alias
        );
    }
    let _ = writeln!(out, "  }}");
    let _ = writeln!(out, "  links {{");
    let mut links = coding.links.clone();
    links.sort_by(|a, b| a.id.cmp(&b.id));
    for link in &links {
        let order = match link.order {
            Order::None => "none",
            Order::Ordered => "ordered",
        };
        let _ = writeln!(out, "    link {} order {order} {{", link.id);
        let mut members = link.members.clone();
        if link.order == Order::None {
            members.sort();
        }
        for member in &members {
            let mark = match member.mark {
                Mark::None => "",
                Mark::Tail => " tail",
                Mark::Head => " head",
            };
            let _ = writeln!(
                out,
                "      {}.{}@{}{mark}",
                member.body, member.instance, member.port
            );
        }
        let _ = writeln!(out, "    }}");
    }
    let _ = writeln!(out, "  }}");
    let _ = writeln!(out, "  lenses {{");
    let mut lenses = coding.lenses.clone();
    lenses.sort_by(|a, b| a.name.cmp(&b.name));
    for lens in &lenses {
        let _ = writeln!(out, "    lens {} {{", lens.name);
        let mut galaxies = lens.galaxies.clone();
        galaxies.sort_by(|a, b| a.name.cmp(&b.name));
        for galaxy in &galaxies {
            let _ = writeln!(out, "      galaxy {} {{", galaxy.name);
            let mut systems = galaxy.systems.clone();
            systems.sort_by(|a, b| a.name.cmp(&b.name));
            for system in &systems {
                let mut aliases = system.bodies.clone();
                aliases.sort();
                let _ = writeln!(
                    out,
                    "        system {} {{ {} }}",
                    system.name,
                    aliases.join(" ")
                );
            }
            let _ = writeln!(out, "      }}");
        }
        let _ = writeln!(out, "    }}");
    }
    let _ = writeln!(out, "  }}");
    let _ = writeln!(out, "}}");
    out
}
