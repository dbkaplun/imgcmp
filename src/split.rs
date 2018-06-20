use image::{GenericImageView, SubImage};

#[derive(Clone, Copy)]
pub enum SplitDirection {
    Horiz,
    Vert,
}

#[derive(Clone, Copy)]
pub struct Split {
    pub dir: SplitDirection,
    pub at: u32,
    pub size: u32,
}

impl Split {
    pub fn split<'i, I: GenericImageView>(
        &self,
        img: &'i I,
    ) -> (
        SubImage<&'i I::InnerImageView>,
        SubImage<&'i I::InnerImageView>,
    ) {
        let (x, y, w, h) = img.bounds();
        match self.dir {
            SplitDirection::Horiz => (
                img.view(x, y, self.at, h),
                img.view(x + self.at, y, w - self.at, h),
            ),
            SplitDirection::Vert => (
                img.view(x, y, w, self.at),
                img.view(x, y + self.at, w, h - self.at),
            ),
        }
    }
}
