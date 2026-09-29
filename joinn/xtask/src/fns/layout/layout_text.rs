//! `cargo xtask layout <path>`: one corpus body's layout block.

use joinn_frame::Verdict;
use joinn_visual::layout;

use super::corpus_bodies::corpus_bodies;
use super::layout_block::layout_block;

/// `rel` is corpus-relative (`phase2/calculator.body`). A refusal is an error.
pub(crate) fn layout_text(rel: &str) -> Result<String, String> {
    let rel = rel.replace('\\', "/");
    let rel = rel.strip_prefix("corpus/").unwrap_or(&rel);
    let Some((_, bound)) = corpus_bodies()?.into_iter().find(|(r, _)| r == rel) else {
        return Err(format!(
            "layout: {rel} is not a corpus .body; acceptance is a path under corpus/"
        ));
    };
    let (body, cells) = match bound {
        Verdict::Ok(bound) => bound,
        Verdict::Refused(r) => return Err(format!("{rel}: {}", r.reason)),
    };
    match layout(&body, &cells) {
        Verdict::Ok(l) => Ok(layout_block(&l)),
        Verdict::Refused(r) => Err(format!("{rel}: {}", r.reason)),
    }
}

#[cfg(test)]
mod tests {
    use super::layout_text;

    #[test]
    fn the_calculator_prints_the_plan_block_and_four_cameras() {
        let got = layout_text("phase2/calculator.body").unwrap_or_else(|e| panic!("{e}"));
        let want = "\
layout body
surface 0 0 40 24 r 3
cell cli_a 4 4 12 6 r 2
cell cli_b 4 14 12 6 r 2
cell sum 24 4 12 10 r 2
port cli_a@0 in 4 7 r 1
port cli_a@1 out 16 7 r 1
port cli_b@0 in 4 17 r 1
port cli_b@1 out 16 17 r 1
port sum@0 in 24 7 r 1
port sum@1 in 24 11 r 1
port sum@2 out 36 7 r 1
wire cli_a@1 -> sum@0 16 7 24 7 w 1/4
wire cli_b@1 -> sum@1 16 17 24 11 w 1/4
camera 640x360: k 12, origin 80 36
camera 1000x777: k 24, origin 20 100
camera 1280x720: k 28, origin 80 24
camera 1920x1080: k 40, origin 160 60
";
        assert_eq!(got, want);
    }
}
