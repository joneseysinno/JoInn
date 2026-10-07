//! Enter on a waiting box: the body grows by what was typed, or refuses it.

use joinn_frame::{Frame, IntFrame, Term, TextFrame, Verdict};

use super::{Grower, NOTHING_SENT};

impl Grower {
    /// `intent numbers.<n> "<text>"`, then `grew numbers.<n>; count <c>` with the
    /// next waiting box selected, or `refused: <reason>`: nothing grows, no row
    /// is written and the selection stays. Text that is not an integer reaches
    /// the body as text, which it refuses as not in ℤ.
    pub fn enter(&mut self) -> Vec<String> {
        if self.selected.is_none() {
            return Vec::new();
        }
        if self.buffer.is_empty() {
            return vec![NOTHING_SENT.to_owned()];
        }
        let text = std::mem::take(&mut self.buffer);
        let n = self.scene.grown().inputs().len();
        let instance = self.scene.grown().instance(n);
        let mut lines = vec![format!("intent {instance} \"{text}\"")];
        let value = match text.trim().parse::<i64>() {
            Ok(v) => IntFrame::new().canonicalize(Term::int(v)),
            Err(_) => TextFrame::new().canonicalize(Term::text(text.clone())),
        };
        let grew = match value {
            Verdict::Ok(v) => self.scene.apply_growth(&v),
            Verdict::Refused(r) => Verdict::Refused(r),
        };
        match grew {
            Verdict::Ok(_) => {
                lines.push(format!(
                    "grew {instance}; count {}",
                    self.scene.count().print_term()
                ));
                self.selected = Some(self.scene.grown().instance(n + 1));
            }
            Verdict::Refused(r) => lines.push(format!("refused: {}", r.reason)),
        }
        lines
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use joinn_visual::Rect;

    use super::super::system_fixture::grower;
    use super::super::{Grower, View};

    /// Home, as the key does (the body has grown past the opening frame), then
    /// the pixel at the centre of `r` (layout units).
    fn centre(g: &mut Grower, r: Rect) -> (u32, u32) {
        let home = match g.home(1280, 720) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert_eq!(g.set_camera(home), Verdict::Ok(()));
        let f = g.scene.fit(1280, 720);
        let px = |o: i64, v2: i64| u32::try_from(o + f.k * v2 / 2).unwrap_or(0);
        (px(f.ox, 2 * r.x + r.w), px(f.oy, 2 * r.y + r.h))
    }

    /// Click the first waiting box, type `text`, Enter.
    fn grow_by(g: &mut Grower, text: &str) -> Vec<String> {
        let Some(b) = g.scene.layout().waiting.first().map(|b| b.rect) else {
            panic!("no waiting box");
        };
        let (x, y) = centre(g, b);
        let clicked = g.click(x, y);
        assert!(
            clicked.iter().any(|l| l.starts_with("selected numbers.")),
            "{clicked:?}"
        );
        for ch in text.chars() {
            assert!(g.type_char(&ch.to_string()).is_some());
        }
        g.enter()
    }

    #[test]
    fn counting_grows_to_three_refuses_three_and_its_lasso_names_the_force() {
        let mut g = grower("counting");
        for n in 0..3 {
            let lines = grow_by(&mut g, "1");
            assert_eq!(
                lines,
                vec![
                    format!("intent numbers.{n} \"1\""),
                    format!("grew numbers.{n}; count {}", n + 1)
                ]
            );
            assert!(g.take_pending().is_some(), "a growth writes rows");
        }
        let lines = grow_by(&mut g, "3");
        assert_eq!(
            lines,
            vec![
                "intent numbers.3 \"3\"".to_owned(),
                "refused: counting: 3 is not one; acceptance is 1 (counting grows by one)"
                    .to_owned()
            ]
        );
        assert_eq!(g.take_pending(), None, "a refusal writes no row");
        assert!(g.typing(), "the selection stays");
        let Some(neck) = g.scene.layout().lasso.get(8).copied() else {
            panic!("the lasso has a neck");
        };
        let mid = Rect {
            x: (neck.from.0 + neck.to.0) / 2,
            y: neck.from.1,
            w: 0,
            h: 0,
        };
        let (x, y) = centre(&mut g, mid);
        assert_eq!(
            g.click(x, y),
            vec![format!("pick {x},{y}: force count (cpu)")]
        );
    }

    #[test]
    fn adding_grows_two_three_four_to_nine() {
        let mut g = grower("adding");
        let mut last = Vec::new();
        for v in ["2", "3", "4"] {
            last = grow_by(&mut g, v);
        }
        assert_eq!(
            last.last().map(String::as_str),
            Some("grew numbers.2; count 9")
        );
    }
}
