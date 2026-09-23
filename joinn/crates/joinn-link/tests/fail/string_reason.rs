fn main() {
    let _ = joinn_link::LinkRefusal {
        link: String::new(),
        body: String::new(),
        member: joinn_link::Address {
            instance: String::new(),
            port: 0,
        },
        kind: joinn_link::LinkRefusalKind::Refused,
        reason: String::from("secret membrane detail"),
    };
}
