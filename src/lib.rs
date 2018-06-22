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
}

#[cfg(test)]
mod tests {
    use super::Node;
    use image;
    use splitter::SplitInHalf;

    #[test]
    fn it_works() {
        let img = image::open("./fixtures/mandrill.png").unwrap();
        println!("color {:?}", Node::new::<SplitInHalf, _>(&img).color)
    }
}
