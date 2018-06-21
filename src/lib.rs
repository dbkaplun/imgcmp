extern crate image;
extern crate num_traits;

use image::{GenericImageView, Pixel, SubImage};
use num_traits::ToPrimitive;

mod util;

use util::newtons_method;

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
    fn split<'i, I: GenericImageView>(&self, image: &'i I) -> (SubImage<&'i I::InnerImageView>,
                                                               SubImage<&'i I::InnerImageView>) {
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
    fn split_default<I: GenericImageView<Pixel = P>>(image: &I) -> Self {
        Self::new_from_image_and_split::<DefaultSplit, _>(image)
    }

    fn new_from_image_and_split<F: Splitter, I: GenericImageView<Pixel = P>>(
        image: &I,
    ) -> Self
    {
        let split = F::split(image);
        Self {
            color: unimplemented!(), // image.color_avg(),
            // child: None,
            child: if split.size > 1 {
                let (a, b) = split.split(image);
                Some(Box::new(NodeChild {
                    a: Self::new_from_image_and_split::<F, _>(&a),
                    b: Self::new_from_image_and_split::<F, _>(&b),
                    split,
                }))
            } else {
                None
            },
        }
    }
}

trait Splitter {
    fn split<I: GenericImageView>(image: &I) -> Split
    where
        <I::Pixel as Pixel>::Subpixel: ToPrimitive;
}

struct DefaultSplit;
impl Splitter for DefaultSplit {
    fn split<I: GenericImageView>(image: &I) -> Split
    where
        <I::Pixel as Pixel>::Subpixel: ToPrimitive
    {
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
}

struct BestSplit;

impl Splitter for BestSplit {
    fn split<I: GenericImageView>(image: &I) -> Split
    where
        <I::Pixel as Pixel>::Subpixel: ToPrimitive
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
                            image.view(0, 0, split, h),
                            image.view(split, 0, w - split, h),
                        )
                    }
                    SplitDirection::Vert => {
                        let split = (n * h as f64) as u32;
                        (
                            image.view(0, 0, w, split),
                            image.view(0, split, w, h - split),
                        )
                    }
                };
                unimplemented!() //color_dist2(a.color_avg(), b.color_avg()).to_f64().unwrap()
            },
            0.5,
        ).unwrap();
        Split {
            dir,
            size,
            at: (split_frac * size as f64) as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
