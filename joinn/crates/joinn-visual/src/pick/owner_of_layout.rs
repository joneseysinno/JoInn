//! Name the owner of an ID drawn from `shapes_of_layout`.

use joinn_frame::Verdict;
use joinn_link::Address;

use super::{Owner, PORT_TAG, TAG_MASK, WIRE_TAG};
use crate::layout::Layout;
use crate::refuse::refuse;

/// Decode `[body + 1, cell + 1, tag | n, generation]` against a fresh layout.
pub fn owner_of_layout(layout: &Layout, id: [u32; 4]) -> Verdict<Owner> {
    let [body, cell, b, generation] = id;
    if id == [0; 4] {
        return Verdict::Ok(Owner::Background);
    }
    let unknown = || {
        refuse(format!(
            "pick: ID {id:?} names nothing in this layout; acceptance is an ID drawn from its shapes"
        ))
    };
    if body != 1 || generation != 1 {
        return unknown();
    }
    let n = b & !TAG_MASK;
    match (cell, b & TAG_MASK) {
        (0, 0) if b == 0 => Verdict::Ok(Owner::Membrane),
        (0, WIRE_TAG) => match layout.wires.get(n as usize) {
            Some(w) => Verdict::Ok(Owner::Wire {
                src: w.src.clone(),
                dst: w.dst.clone(),
            }),
            None => unknown(),
        },
        (0, _) => unknown(),
        (c, tag) => {
            let Some(cell) = layout.cells.get(c as usize - 1) else {
                return unknown();
            };
            match tag {
                0 if b == 0 => Verdict::Ok(Owner::Cell(cell.instance.clone())),
                PORT_TAG
                    if layout
                        .ports
                        .iter()
                        .any(|p| p.address.instance == cell.instance && p.address.port == n) =>
                {
                    Verdict::Ok(Owner::Port(Address {
                        instance: cell.instance.clone(),
                        port: n,
                    }))
                }
                _ => unknown(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use super::owner_of_layout;
    use crate::camera::fit;
    use crate::fixtures::calculator;
    use crate::layout::layout;
    use crate::pick::{Pick, cpu_pick, print_owner, shapes_of_layout};

    #[test]
    fn the_probe_pixels_name_the_plan_owners_on_the_cpu() {
        let (body, cells) = calculator();
        let l = match layout(&body, &cells) {
            Verdict::Ok(l) => l,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let camera = fit(&l, 1280, 720);
        assert_eq!((camera.k, camera.ox, camera.oy), (28, 80, 24));
        let image = match cpu_pick(&shapes_of_layout(&l), &camera) {
            Verdict::Ok(p) => p,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let probes = [
            (136, 80, "body membrane"),
            (360, 220, "body.cli_a"),
            (360, 500, "body.cli_b"),
            (920, 276, "body.sum"),
            (192, 220, "body.cli_a@0"),
            (528, 220, "body.cli_a@1"),
            (192, 500, "body.cli_b@0"),
            (528, 500, "body.cli_b@1"),
            (752, 220, "body.sum@0"),
            (752, 332, "body.sum@1"),
            (1088, 220, "body.sum@2"),
            (640, 220, "body wire cli_a@1 -> sum@0"),
            (640, 416, "body wire cli_b@1 -> sum@1"),
            (0, 0, "background"),
        ];
        for (x, y, want) in probes {
            let pick = image.pixels[(y * 1280 + x) as usize];
            let got = match pick {
                Pick::Background => "background".to_owned(),
                Pick::Edge => "edge".to_owned(),
                Pick::Owned(id) => match owner_of_layout(&l, id) {
                    Verdict::Ok(o) => print_owner("body", &o),
                    Verdict::Refused(r) => panic!("{}", r.reason),
                },
            };
            assert_eq!(got, want, "pixel {x},{y}");
        }
    }

    #[test]
    fn an_id_the_layout_never_drew_is_refused() {
        let (body, cells) = calculator();
        let l = match layout(&body, &cells) {
            Verdict::Ok(l) => l,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        for id in [
            [1, 9, 0, 1],
            [1, 0, 0x2000_0005, 1],
            [1, 1, 0x1000_0007, 1],
            [2, 0, 0, 1],
        ] {
            assert!(
                matches!(owner_of_layout(&l, id), Verdict::Refused(_)),
                "{id:?}"
            );
        }
    }
}
