extern crate image;
extern crate num_traits;

use image::{GenericImageView, Pixel};

mod split;
mod splitter;
mod util;

use splitter::Splitter;

struct Node<P: Pixel> {
    color: P,
    child: Option<Box<NodeChild<P>>>,
}

struct NodeChild<P: Pixel> {
    a: Node<P>,
    b: Node<P>,
    split: split::Split,
}

/// An enum used for controlling the execution of `.fold_while()`.
///
/// See `.fold_while()` for more information.
#[derive(Copy, Clone, Debug)]
pub enum FoldWhile<T> {
    /// Continue folding with this value
    Continue(T),
    /// Fold is complete and will return this value
    Done(T),
}

impl<T> FoldWhile<T> {
    /// Return the value in the continue or done.
    pub fn into_inner(self) -> T {
        match self {
            FoldWhile::Continue(x) | FoldWhile::Done(x) => x,
        }
    }

    /// Return true if `self` is `Done`, false if it is `Continue`.
    pub fn is_done(&self) -> bool {
        match *self {
            FoldWhile::Continue(_) => false,
            FoldWhile::Done(_) => true,
        }
    }
}

impl<P: Pixel> Node<P> {
    fn new<S: Splitter, I: GenericImageView<Pixel = P>>(img: &I) -> Self {
        let img_split = S::split(img);
        Self {
            color: util::avg_color(img),
            child: if img_split.size > 1 {
                let (a, b) = img_split.split(img);
                Some(Box::new(NodeChild {
                    a: Self::new::<S, _>(&a),
                    b: Self::new::<S, _>(&b),
                    split: img_split,
                }))
            } else {
                None
            },
        }
    }

    fn fold_while<F, T>(&self, f: &F) -> FoldWhile<T>
    where
        F: Fn(&Self, Option<(T, T)>) -> FoldWhile<T>,
    {
        if let Some(child) = &self.child {
            let NodeChild { ref a, ref b, .. } = **child;
            let af = a.fold_while(&f);
            if af.is_done() {
                return af;
            }
            let bf = b.fold_while(&f);
            if bf.is_done() {
                return bf;
            }

            f(self, Some((af.into_inner(), bf.into_inner())))
        } else {
            f(self, None)
        }
    }

    fn fold<F, T>(&self, f: &F) -> T
    where
        F: Fn(&Self, Option<(T, T)>) -> T,
    {
        self.fold_while(&|node, item| FoldWhile::Continue(f(node, item)))
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::{Node, NodeChild};
    use image;
    use split::SplitDirection;
    use splitter::SplitInHalf;

    #[test]
    fn it_works() {
        let img = image::open("./fixtures/mandrill.png").unwrap();
        let root = Node::new::<SplitInHalf, _>(&img);
        let folded = root.fold(&|node, item: Option<(image::ImageBuffer<_, _>, _)>| {
            match (&node.child, &item) {
                (None, None) => image::ImageBuffer::from_pixel(1, 1, node.color),
                (Some(child), Some((a, b))) => {
                    let NodeChild { split, .. } = **child;
                    let (w, h) = split.dimensions();
                    // match split.dir {
                    //     SplitDirection::Horiz => {
                    //         assert_eq!(split.at, a.width());
                    //         assert_eq!(split.size, a.width() + b.width());
                    //         assert_eq!(split.size_orthogonal, a.height());
                    //         assert_eq!(split.size_orthogonal, b.height());
                    //     }
                    //     SplitDirection::Vert => {
                    //         assert_eq!(split.at, a.height());
                    //         assert_eq!(split.size, a.height() + b.height());
                    //         assert_eq!(split.size_orthogonal, a.width());
                    //         assert_eq!(split.size_orthogonal, b.width());
                    //     }
                    // }

                    image::ImageBuffer::from_fn(w, h, |x, y| {
                        match split.dir {
                            SplitDirection::Horiz => {
                                if x < split.at {
                                    a.get_pixel(x, y)
                                } else {
                                    b.get_pixel(x - split.at, y)
                                }
                            }
                            SplitDirection::Vert => {
                                if y < split.at {
                                    a.get_pixel(x, y)
                                } else {
                                    b.get_pixel(x, y - split.at)
                                }
                            }
                        }.clone()
                    })
                }
                _ => panic!(),
            }
        });
        folded.save("out.png").unwrap();
    }
}
