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
                size_orthogonal: h,
            }
        } else {
            Split {
                dir: SplitDirection::Vert,
                at: h / 2,
                size: h,
                size_orthogonal: w,
            }
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct MaximalColorDifferenceSplitter;

impl Splitter for MaximalColorDifferenceSplitter {
    fn split<I: GenericImageView>(img: &I) -> Split {
        let (w, h) = img.dimensions();
        let mut split = if w > h {
            Split {
                dir: SplitDirection::Horiz,
                at: 0,
                size: w,
                size_orthogonal: h,
            }
        } else {
            Split {
                dir: SplitDirection::Vert,
                at: 0,
                size: h,
                size_orthogonal: w,
            }
        };
        split.at = (f64::from(split.size)
            * newtons_method(
                &|n| {
                    let (a, b) = match split.dir {
                        SplitDirection::Horiz => {
                            let at = (n * f64::from(w)) as u32;
                            (img.view(0, 0, at, h), img.view(at, 0, w - at, h))
                        }
                        SplitDirection::Vert => {
                            let at = (n * f64::from(h)) as u32;
                            (img.view(0, 0, w, at), img.view(0, at, w, h - at))
                        }
                    };
                    color_dist2(avg_color(&a), avg_color(&b)).to_f64().unwrap()
                },
                0.5,
            ).unwrap()) as u32;
        split
    }
}
