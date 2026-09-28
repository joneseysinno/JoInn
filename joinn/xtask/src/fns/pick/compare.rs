//! Compare a GPU ID image with a CPU pick image, pixel by pixel.

use joinn_visual::{Pick, PickImage};

use super::{KEPT, Tally};

/// An edge pixel is counted and never judged. A GPU image of the wrong length
/// leaves pixels uncounted, which the caller's sum check refuses.
pub(crate) fn compare(cpu: &PickImage, gpu: &[[u32; 4]]) -> Tally {
    let mut t = Tally::default();
    for (i, (pick, id)) in cpu.pixels.iter().zip(gpu).enumerate() {
        let want = match pick {
            Pick::Edge => {
                t.edge += 1;
                continue;
            }
            Pick::Background => [0; 4],
            Pick::Owned(owner) => *owner,
        };
        if want == *id {
            t.agree += 1;
            if want != [0; 4] {
                t.owners.insert(want);
            }
        } else {
            t.disagree += 1;
            if t.first.len() < KEPT {
                t.first.push((i, want, *id));
            }
        }
    }
    t
}

#[cfg(test)]
mod tests {
    use joinn_visual::{Pick, PickImage};

    use super::compare;

    #[test]
    fn every_pixel_is_agree_edge_or_disagree() {
        let owner = [1, 1, 0, 1];
        let cpu = PickImage {
            width: 4,
            height: 1,
            pixels: vec![
                Pick::Background,
                Pick::Edge,
                Pick::Owned(owner),
                Pick::Owned(owner),
            ],
        };
        let gpu = [[0; 4], [9, 9, 9, 9], owner, [1, 2, 0, 1]];
        let t = compare(&cpu, &gpu);
        assert_eq!((t.agree, t.edge, t.disagree), (2, 1, 1));
        assert_eq!(t.agree + t.edge + t.disagree, 4);
        assert_eq!(t.first, vec![(3, owner, [1, 2, 0, 1])]);
        assert_eq!(t.owners.len(), 1);
    }

    #[test]
    fn a_short_gpu_image_breaks_the_sum() {
        let cpu = PickImage {
            width: 2,
            height: 1,
            pixels: vec![Pick::Background, Pick::Background],
        };
        let t = compare(&cpu, &[[0; 4]]);
        assert_ne!(t.agree + t.edge + t.disagree, 2);
    }
}
