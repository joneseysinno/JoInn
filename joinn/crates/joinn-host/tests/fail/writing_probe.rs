use joinn_frame::Frame;

fn main() {
    fn writing_probe(state: &joinn_live::BodyState) {
        let value = match joinn_frame::TextFrame::new().canonicalize(joinn_frame::Term::text("1"))
        {
            joinn_frame::Verdict::Ok(v) => v,
            joinn_frame::Verdict::Refused(_) => return,
        };
        let _ = state.inject("sum", 0, value, 0);
    }
    let _ = writing_probe;
}
