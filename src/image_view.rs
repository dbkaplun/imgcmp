use image::{GenericImageView, Pixel, SubImage};
use std::ops::Deref;

use util::color_avg;

/// FIXME: remove and replace when GenericImageView: ?Sized
pub trait ImageView<I: GenericImageView> {
    fn view(&self, x: u32, y: u32, width: u32, height: u32) -> &Self;
    fn color_avg(&self) -> I::Pixel;
    fn bounds(&self) -> (u32, u32, u32, u32);
    fn dimensions(&self) -> (u32, u32) {
        let (x, y, w, h) = self.bounds();
        (w - x, h - y)
    }
}

impl<I: GenericImageView> ImageView<I> for I
where I::InnerImageView: GenericImageView<InnerImageView = I>,
{
    fn bounds(&self) -> (u32, u32, u32, u32) {
        self.bounds()
    }

    fn view(&self, x: u32, y: u32, w: u32, h: u32) -> &Self {
        &self.view(x, y, w, h)
    }

    fn color_avg(&self) -> I::Pixel {
        color_avg(self.pixels().map(|(_, _, px)| px).by_ref()).unwrap()
    }
}

impl<'i, I: GenericImageView + 'i> ImageView<I> for SubImage<&'i I>
where
    I: Deref<Target = &'i I>,
{
    fn bounds(&self) -> (u32, u32, u32, u32) {
        <Self as GenericImageView>::bounds(self)
    }

    fn view(&self, x: u32, y: u32, w: u32, h: u32) -> &Self {
        &<Self as GenericImageView>::view(self, x, y, w, h)
    }

    fn color_avg(&self) -> I::Pixel {
        color_avg(self.pixels().map(|(_, _, px)| px).by_ref()).unwrap()
    }
}
