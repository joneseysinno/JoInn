fn render(&self) -> String { // allow(vocab): planted render fixture, outside the host scan
    let _ = std::fs::read("plant"); // allow(vocab): planted IO fixture, outside the host scan
    String::new()
}
