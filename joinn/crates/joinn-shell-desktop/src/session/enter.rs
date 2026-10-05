//! Enter: a text term, then check, inject, and run.

use joinn_frame::{Frame, TextFrame, Verdict};
use joinn_host::{Host, check_intent, describe_refusal, probe};

use super::{Desktop, NOTHING_SENT};

impl Desktop {
    /// `intent cli_a@0 "2"`, then `fired <instances>` or `refused: <reason>`.
    /// The delta lands in the scene for the next draw. An empty buffer sends
    /// nothing and keeps the selection.
    pub fn enter(&mut self) -> Vec<String> {
        let Some(address) = self.selected.clone() else {
            return Vec::new();
        };
        if self.buffer.is_empty() {
            return vec![NOTHING_SENT.to_owned()];
        }
        let text = std::mem::take(&mut self.buffer);
        let mut lines = Vec::new();
        let intent = match self.intend(text.clone()) {
            Verdict::Ok(Some(intent)) => intent,
            Verdict::Ok(None) => return lines,
            Verdict::Refused(r) => {
                lines.push(format!("refused: {}", r.reason));
                return lines;
            }
        };
        if let Verdict::Refused(r) = check_intent(&self.body, &self.cells, &intent) {
            lines.push(format!("refused: {}", r.reason));
            return lines;
        }
        lines.push(format!("intent {} \"{text}\"", address.printed()));
        let value = match TextFrame::new().canonicalize(intent.term) {
            Verdict::Ok(value) => value,
            Verdict::Refused(r) => {
                lines.push(format!("refused: {}", r.reason));
                return lines;
            }
        };
        if let Verdict::Refused(r) =
            self.state
                .inject(&address.instance, address.port, value, self.epoch)
        {
            lines.push(format!("refused: {}", r.reason));
            return lines;
        }
        self.epoch += 1;
        match self.state.run() {
            Verdict::Ok(reports) => {
                let mut fired = Vec::new();
                for report in &reports {
                    if let Some(name) = &report.fired {
                        if !fired.contains(name) {
                            fired.push(name.clone());
                        }
                    }
                }
                if fired.is_empty() {
                    lines.push("fired".to_owned());
                } else {
                    lines.push(format!("fired {}", fired.join(" ")));
                }
                if let Verdict::Refused(r) = self.scene.apply_run(&reports, &self.state) {
                    lines.push(format!("refused: {}", r.reason));
                }
            }
            Verdict::Refused(refusal) => {
                let reason = match probe(&self.state, address.clone()) {
                    Verdict::Ok(d) if !d.label.is_empty() => d.label,
                    _ => refusal.reason.clone(),
                };
                lines.push(format!("refused: {reason}"));
                let description = describe_refusal(&self.state, &address.instance, &refusal.reason);
                if let Verdict::Refused(r) = self.present(&description) {
                    lines.push(format!("refused: {}", r.reason));
                }
            }
        }
        lines
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use joinn_visual::{Scene, table_bytes};

    use super::super::contact_fixture::calculator_contact;
    use super::super::fixtures::calculator;
    use super::super::{Desktop, View};

    /// The script at 1280×720: the pixel of the in-port, then the text typed there.
    const SCRIPT: [((u32, u32), &str); 3] =
        [((192, 220), "two"), ((192, 220), "2"), ((192, 500), "3")];

    /// The same script on the contact calculator: cli_a@0 and cli_b@0 sit at
    /// layout (4, 7) and (4, 13), pixels (256, 264) and (256, 456) at k 32.
    const CONTACT_SCRIPT: [((u32, u32), &str); 3] =
        [((256, 264), "two"), ((256, 264), "2"), ((256, 456), "3")];

    /// V125: click, type and Enter through the shell's own leaves; after each
    /// Enter the session's tables equal the scene regrown from its live state,
    /// byte for byte. Returns the rows each Enter wrote.
    fn run_path_rows(
        desktop: &mut Desktop,
        script: &[((u32, u32), &str)],
        regrow: &dyn Fn(&Desktop) -> Verdict<Scene>,
    ) -> Vec<usize> {
        let mut rows = Vec::new();
        for (step, ((x, y), text)) in script.iter().enumerate() {
            let clicked = desktop.click(*x, *y);
            assert!(
                clicked.iter().any(|line| line.starts_with("selected ")),
                "step {step}: {clicked:?}"
            );
            for ch in text.chars() {
                let typed = desktop.type_char(&ch.to_string());
                assert!(typed.is_some(), "step {step}: typing {ch}");
            }
            let lines = desktop.enter();
            assert!(!lines.is_empty(), "step {step}: Enter ran nothing");
            let regrown = match regrow(desktop) {
                Verdict::Ok(scene) => scene,
                Verdict::Refused(r) => panic!("step {step}: {}", r.reason),
            };
            assert_eq!(
                table_bytes(desktop.tables()),
                table_bytes(regrown.tables()),
                "step {step} {text:?}: V122 on the shell's run path; lines {lines:?}"
            );
            let delta = desktop.take_pending().unwrap_or_default();
            rows.push(delta.rows.len());
        }
        rows
    }

    #[test]
    fn the_shell_run_path_keeps_tables_equal_to_regrow_on_the_calculator() {
        let mut desktop = calculator();
        let rows = run_path_rows(&mut desktop, &SCRIPT, &|d| {
            Scene::regrow("body", &d.body, &d.cells, &d.state)
        });
        let printed: Vec<String> = rows.iter().map(usize::to_string).collect();
        println!("calculator.body rows {}", printed.join(", "));
        assert_eq!(rows, vec![2, 3, 4]);
    }

    #[test]
    fn the_shell_run_path_keeps_tables_equal_to_regrow_on_the_contact_calculator() {
        let (mut desktop, contact) = calculator_contact();
        let rows = run_path_rows(&mut desktop, &CONTACT_SCRIPT, &|d| {
            Scene::regrow_contact("body", &contact, &d.cells, &d.state)
        });
        let printed: Vec<String> = rows.iter().map(usize::to_string).collect();
        println!("calculator.contact rows {}", printed.join(", "));
        assert_eq!(rows, vec![2, 1, 3]);
    }

    #[test]
    fn clicking_sum_in_the_contact_picture_names_the_response() {
        let (mut desktop, _) = calculator_contact();
        let lines = desktop.click(832, 360);
        assert_eq!(lines, vec!["pick 832,360: body.sum (cpu)".to_owned()]);
    }

    #[test]
    fn enter_on_an_empty_buffer_sends_nothing_and_keeps_the_selection() {
        let mut desktop = calculator();
        let selected = desktop.click(192, 220);
        assert!(
            selected.iter().any(|line| line == "selected body.cli_a@0"),
            "{selected:?}"
        );
        let lines = desktop.enter();
        println!("{lines:?}");
        assert_eq!(lines, vec!["  (empty: nothing sent)".to_owned()]);
        assert_eq!(desktop.take_pending(), None, "no run, so no delta");
        let described = match joinn_host::describe(&desktop.state, "cli_a") {
            Verdict::Ok(d) => d,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let fresh = calculator();
        let untouched = match joinn_host::describe(&fresh.state, "cli_a") {
            Verdict::Ok(d) => d,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert_eq!(described, untouched, "no intent reached cli_a");
        assert_eq!(
            desktop.type_char("2").as_deref(),
            Some("  typing: 2"),
            "the selection stays"
        );
        let lines = desktop.enter();
        assert!(
            lines.iter().any(|line| line == "intent cli_a@0 \"2\""),
            "{lines:?}"
        );
    }

    #[test]
    fn typing_2_then_enter_on_cli_a_yields_the_intent() {
        let mut desktop = calculator();
        let selected = desktop.click(192, 220);
        assert!(
            selected.iter().any(|line| line == "selected body.cli_a@0"),
            "{selected:?}"
        );
        assert_eq!(desktop.type_char("2").as_deref(), Some("  typing: 2"));
        let lines = desktop.enter();
        assert!(
            lines.iter().any(|line| line == "intent cli_a@0 \"2\""),
            "{lines:?}"
        );
    }
}
