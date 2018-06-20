extern crate image;
extern crate num_traits;

use image::{GenericImageView, Pixel};
use num_traits::ToPrimitive;

mod image_view;
mod util;

use image_view::ImageView;
use util::{color_dist2, newtons_method};

#[derive(Clone, Copy)]
enum SplitDirection {
    Horiz,
    Vert,
}

#[derive(Clone, Copy)]
struct Split {
    dir: SplitDirection,
    at: u32,
    size: u32,
}

impl Split {
    fn split<'i, I: GenericImageView, V: ImageView<I>>(&self, image: &'i V) -> (&'i V, &'i V) {
        let (x, y, w, h) = image.bounds();
        match self.dir {
            SplitDirection::Horiz => (
                image.view(x, y, self.at, h),
                image.view(x + self.at, y, w - self.at, h),
            ),
            SplitDirection::Vert => (
                image.view(x, y, w, self.at),
                image.view(x, y + self.at, w, h - self.at),
            ),
        }
    }
}

struct Node<P: Pixel> {
    color: P,
    child: Option<Box<NodeChild<P>>>,
}

struct NodeChild<P: Pixel> {
    a: Node<P>,
    b: Node<P>,
    split: Split,
}

impl<P: Pixel> Node<P> {
    fn split_default<I: GenericImageView<Pixel = P>, V: ImageView<I>>(image: &V) -> Self {
        Self::new_from_image_and_split(image, &default_split)
    }

    fn new_from_image_and_split<F, I: GenericImageView<Pixel = P>, V: ImageView<I>>(
        image: &V,
        split_fn: &F,
    ) -> Self
    where
        F: Fn(&V) -> Split,
    {
        let split = split_fn(&image);
        Self {
            color: image.color_avg(),
            // child: None,
            child: if split.size > 1 {
                let (a, b) = split.split(image);
                Some(Box::new(NodeChild {
                    a: Self::new_from_image_and_split(a, split_fn),
                    b: Self::new_from_image_and_split(b, split_fn),
                    split,
                }))
            } else {
                None
            },
        }
    }
}

fn default_split<I: GenericImageView, V: ImageView<I>>(image: &V) -> Split {
    let (w, h) = image.dimensions();
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

fn best_split<I: GenericImageView, V: ImageView<I>>(image: &V) -> Split
where
    <I::Pixel as Pixel>::Subpixel: ToPrimitive,
{
    let (w, h) = image.dimensions();
    let (size, dir) = if w > h {
        (w, SplitDirection::Horiz)
    } else {
        (h, SplitDirection::Vert)
    };
    let split_frac = newtons_method(
        &|n| {
            let (a, b) = match dir {
                SplitDirection::Horiz => {
                    let split = (n * w as f64) as u32;
                    (
                        &image.view(0, 0, split, h),
                        &image.view(split, 0, w - split, h),
                    )
                }
                SplitDirection::Vert => {
                    let split = (n * h as f64) as u32;
                    (
                        &image.view(0, 0, w, split),
                        &image.view(0, split, w, h - split),
                    )
                }
            };
            color_dist2(a.color_avg(), b.color_avg()).to_f64().unwrap()
        },
        0.5,
    ).unwrap();
    Split {
        dir,
        size,
        at: (split_frac * size as f64) as u32,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
