use image::GenericImageView;
use num_traits::ToPrimitive;

use split::{Split, SplitDirection};
use util::{avg_color, color_dist2, newtons_method};

pub trait Splitter {
    fn split<I: GenericImageView>(img: &I) -> Split;
}

#[derive(Copy, Clone, Default)]
pub struct SplitInHalf;

impl Splitter for SplitInHalf {
    fn split<I: GenericImageView>(img: &I) -> Split {
        let (w, h) = img.dimensions();
        if w > h {
            Split {
                dir: SplitDirection::Horiz,
                at: w / 2,
                size: w,
            }
        } else {
            Split {
                dir: SplitDirection::Horiz,
                at: h / 2,
                size: h,
            }
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct MaximalColorDifferenceSplitter;

impl Splitter for MaximalColorDifferenceSplitter {
    fn split<I: GenericImageView>(img: &I) -> Split {
        let (w, h) = img.dimensions();
        let (size, dir) = if w > h {
            (w, SplitDirection::Horiz)
        } else {
            (h, SplitDirection::Vert)
        };
        let split_frac = newtons_method(
            &|n| {
                let (a, b) = match dir {
                    SplitDirection::Horiz => {
                        let split = (n * f64::from(w)) as u32;
                        (img.view(0, 0, split, h), img.view(split, 0, w - split, h))
                    }
                    SplitDirection::Vert => {
                        let split = (n * f64::from(h)) as u32;
                        (img.view(0, 0, w, split), img.view(0, split, w, h - split))
                    }
                };
                color_dist2(avg_color(&a).unwrap(), avg_color(&b).unwrap())
                    .to_f64()
                    .unwrap()
            },
            0.5,
        ).unwrap();
        Split {
            dir,
            size,
            at: (split_frac * f64::from(size)) as u32,
        }
    }
}
