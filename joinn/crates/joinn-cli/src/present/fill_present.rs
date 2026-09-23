//! Fill the CLI present template from a description's port values.

use joinn_host::Description;

/// Fill the instance's present template. `None` when the body declares none.
pub(crate) fn fill_present(body: &joinn_dna::Body, d: &Description) -> Option<String> {
    let mut tmpl = body.regulatory.present.get(&d.instance)?.clone();
    for p in &d.ports {
        if let Some(v) = &p.value {
            let needle = format!("{{{}}}", p.position);
            tmpl = tmpl.replace(&needle, v);
        }
    }
    Some(tmpl)
}
