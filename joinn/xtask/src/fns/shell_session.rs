//! The desktop shell's session on the grove, driven without a window
//! (plan 7.2 §2.11, P72-11).

#[cfg(test)]
mod tests {
    use joinn_shell_desktop::{Atlas, Shell, tick_line};

    use crate::fns::corpus_store::corpus_store;
    use crate::fns::grove::{GROVE_SEED, grow_grove};

    fn grove_shell() -> Shell {
        let universe = match grow_grove(GROVE_SEED) {
            Ok(u) => u,
            Err(e) => panic!("{e}"),
        };
        let store = match corpus_store() {
            Ok((_, store)) => store,
            Err(e) => panic!("{e}"),
        };
        match Atlas::open(&universe, store, None, 1920, 1080) {
            Ok(atlas) => Shell::new(Box::new(atlas)),
            Err(e) => panic!("{e}"),
        }
    }

    fn tick(shell: &mut Shell) -> String {
        match shell.take_tick() {
            Some(t) => {
                let rows = t.delta.as_ref().map_or(0, |d| d.rows.len());
                tick_line(&t, rows)
            }
            None => "no tick".to_owned(),
        }
    }

    #[test]
    fn the_grove_frames_zooms_pans_and_resizes_then_idles() {
        let mut shell = grove_shell();
        let mut lines = Vec::new();
        let mut printed = Vec::new();
        printed.extend(shell.frame());
        lines.push(tick(&mut shell));
        for _ in 0..8 {
            printed.extend(shell.wheel(960, 540, 1));
            lines.push(tick(&mut shell));
        }
        shell.press(500, 500);
        printed.extend(shell.drag_to(503, 500));
        assert!(!shell.dirty(), "3 px is not yet a drag");
        printed.extend(shell.drag_to(510, 500));
        printed.extend(shell.release(510, 500));
        lines.push(tick(&mut shell));
        printed.extend(shell.resize(1280, 720));
        lines.push(tick(&mut shell));
        for line in &lines {
            println!("{line}");
        }
        assert_eq!(printed, Vec::<String>::new());
        assert_eq!(
            lines,
            [
                "tick: level -2 step 95 (k 351/1024), anchor universe, rows 0",
                "tick: level -2 step 127 (k 383/1024), anchor universe, rows 0",
                "tick: level -2 step 159 (k 415/1024), anchor universe, rows 0",
                "tick: level -2 step 191 (k 447/1024), anchor universe, rows 0",
                "tick: level -2 step 223 (k 479/1024), anchor universe, rows 0",
                "tick: level -2 step 255 (k 511/1024), anchor universe, rows 0",
                "tick: level -1 step 31 (k 287/512), anchor universe, rows 0",
                "tick: level -1 step 63 (k 319/512), anchor universe, rows 0",
                "tick: level -1 step 95 (k 351/512), anchor universe, rows 0",
                "tick: level -1 step 95 (k 351/512), anchor universe, rows 0",
                "tick: level -1 step 95 (k 351/512), anchor g1s10, rows 3209",
            ]
        );
        assert!(
            shell.take_tick().is_none(),
            "V147: idle after the last input"
        );
        assert!(shell.take_tick().is_none());
    }

    #[test]
    fn a_click_names_its_owner_and_a_zoom_past_level_9_is_refused() {
        let mut shell = grove_shell();
        shell.frame();
        let _ = shell.take_tick();
        shell.press(0, 0);
        let clicked = shell.release(0, 0);
        assert_eq!(clicked, ["pick 0,0: background (cpu)"]);
        assert!(
            shell.take_tick().is_some(),
            "a click owes the confirming tick"
        );
        let mut refused = Vec::new();
        for _ in 0..200 {
            refused = shell.wheel(960, 540, 1);
            if !refused.is_empty() {
                break;
            }
        }
        assert_eq!(
            refused,
            ["refused: zoom: level 10 is outside −4 … 9; acceptance is a level from −4 to 9"]
        );
    }
}
