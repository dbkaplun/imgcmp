use image::{GenericImageView, SubImage};

#[derive(Clone, Copy, Debug)]
pub enum SplitDirection {
    Horiz,
    Vert,
}

#[derive(Clone, Copy, Debug)]
pub struct Split {
    pub dir: SplitDirection,
    pub at: u32,
    pub size: u32,
    pub size_orthogonal: u32,
}

impl Split {
    pub fn split<'i, I: GenericImageView>(
        &self,
        img: &'i I,
    ) -> (
        SubImage<&'i I::InnerImageView>,
        SubImage<&'i I::InnerImageView>,
    ) {
        let (w, h) = img.dimensions();
        match self.dir {
            SplitDirection::Horiz => (
                img.view(0, 0, self.at, h),
                img.view(self.at, 0, w - self.at, h),
            ),
            SplitDirection::Vert => (
                img.view(0, 0, w, self.at),
                img.view(0, self.at, w, h - self.at),
            ),
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        match self.dir {
            SplitDirection::Horiz => (self.size, self.size_orthogonal),
            SplitDirection::Vert => (self.size_orthogonal, self.size),
        }
    }
}
