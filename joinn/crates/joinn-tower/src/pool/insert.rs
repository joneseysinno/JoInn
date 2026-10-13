//! Add a chromosome to the pool.

use joinn_frame::{Hash, Verdict};

use super::{Pool, cycle, short};
use crate::chromosome::Chromosome;
use crate::refuse::refuse;

impl Pool {
    /// Its hash. A chromosome that reaches itself through references is
    /// refused; so is a second chromosome under a held hash with other
    /// references. The same chromosome twice is held once.
    pub fn insert(&mut self, c: Chromosome) -> Verdict<Hash> {
        let h = c.identity();
        if let Some(held) = self.chromosomes.get(&h) {
            if *held == c {
                return Verdict::Ok(h);
            }
            return refuse(format!(
                "pool: gene {} is held with other references; acceptance is one chromosome per hash",
                short(&h)
            ));
        }
        if let Some(path) = cycle(&self.chromosomes, &c) {
            let names: Vec<String> = path.iter().map(short).collect();
            return refuse(format!(
                "pool: gene {} contains itself through {}; acceptance is a pool without cycles",
                short(&h),
                names.join(" → ")
            ));
        }
        self.chromosomes.insert(h, c);
        Verdict::Ok(h)
    }
}
