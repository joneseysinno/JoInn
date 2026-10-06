//! Open a universe: admit it, lay out one lens, grow its tables, frame it.

use joinn_frame::Verdict;
use joinn_link::{
    BodyStore, Universe, assemble_universe, bind, canonical_universe, check_law4, check_lenses,
    check_link_types,
};
use joinn_visual::{Camera, ChartId, Rect, UniverseScene, layout_universe};

use super::{Atlas, View};

impl Atlas {
    /// Admitted as any universe: bodies bound from `store` by hash, then Law 4,
    /// lenses, link types and assembly, then put in print order
    /// (`canonical_universe`). `lens`, or else the first lens in canonical
    /// order, is laid out and grown, and framed in `width` × `height`.
    pub fn open(
        universe: &Universe,
        store: &BodyStore,
        lens: Option<&str>,
        width: u32,
        height: u32,
    ) -> Result<Atlas, String> {
        let bound = match bind(universe, store) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => return Err(r.reason),
        };
        for verdict in [check_law4(universe), check_lenses(universe)] {
            if let Verdict::Refused(r) = verdict {
                return Err(r.reason);
            }
        }
        if let Verdict::Refused(r) = check_link_types(universe, &bound) {
            return Err(r.reason);
        }
        if let Verdict::Refused(r) = assemble_universe(universe, &bound) {
            return Err(r.reason);
        }
        let universe = &canonical_universe(universe);
        let mut names: Vec<&str> = universe
            .coding
            .lenses
            .iter()
            .map(|l| l.name.as_str())
            .collect();
        names.sort_unstable();
        let Some(name) = lens.or(names.first().copied()) else {
            return Err(
                "open: the universe declares no lens; acceptance is a universe with a lens"
                    .to_owned(),
            );
        };
        let layout = match layout_universe(universe, store, name) {
            Verdict::Ok(l) => l,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let scene = match UniverseScene::grow(layout, ChartId(0)) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let (w, h) = scene.layout().chart(ChartId(0)).map_or((0, 0), |c| c.size);
        let home = Camera::frame(Rect { x: 0, y: 0, w, h }, ChartId(0), width, height);
        let mut atlas = Atlas {
            scene,
            camera: home,
            confirm: None,
        };
        if let Verdict::Refused(r) = atlas.set_camera(home) {
            return Err(r.reason);
        }
        let _ = atlas.scene.take_pending();
        Ok(atlas)
    }
}

#[cfg(test)]
mod tests {
    use joinn_link::BodyStore;

    use super::super::{Atlas, View};
    use crate::load::{find_corpus, load_cells, load_store, load_universe_file};

    fn phase5() -> (joinn_link::Universe, BodyStore) {
        let corpus = match find_corpus() {
            Ok(c) => c,
            Err(e) => panic!("{e}"),
        };
        let cells = match load_cells(&corpus) {
            Ok(c) => c,
            Err(e) => panic!("{e}"),
        };
        let store = match load_store(&corpus, &cells) {
            Ok(s) => s,
            Err(e) => panic!("{e}"),
        };
        match load_universe_file(&corpus.join("phase5").join("universe.universe")) {
            Ok(u) => (u, store),
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn the_first_lens_in_canonical_order_opens_unless_one_is_named() {
        let (universe, store) = phase5();
        let lens = |name: Option<&str>| match Atlas::open(&universe, &store, name, 1280, 720) {
            Ok(atlas) => (atlas.scene.layout().lens.clone(), atlas.anchor_name()),
            Err(e) => (e, String::new()),
        };
        assert_eq!(lens(None), ("deployment".to_owned(), "universe".to_owned()));
        assert_eq!(
            lens(Some("function")),
            ("function".to_owned(), "universe".to_owned())
        );
        assert_eq!(
            lens(Some("nowhere")).0,
            "layout: lens nowhere not found; acceptance is a lens the universe declares"
        );
    }

    #[test]
    fn a_body_the_corpus_does_not_hold_is_refused_by_alias_and_hash() {
        let (universe, _) = phase5();
        let refused = match Atlas::open(&universe, &BodyStore::new(), None, 1280, 720) {
            Ok(_) => String::new(),
            Err(e) => e,
        };
        assert_eq!(
            refused,
            "alias calc declared b55fba1eff65942099f6daf84b8bc47d605be05f637d805d260d3fcfd8c3ebde and the store holds no such body"
        );
    }
}
