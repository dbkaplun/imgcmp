use image::{GenericImageView, Pixel};
use num_traits::{Float, One, Zero};
use std::error::Error;

pub fn newtons_method<N: Float, F: Fn(N) -> N>(f: &F, guess: N) -> Result<N, Box<Error>> {
    let mut cur_guess = guess;
    for _ in 0.. {
        let prev_guess = cur_guess;

        let fguess = f(cur_guess);
        if fguess.abs() <= N::epsilon() {
            // found a root!
            return Ok(cur_guess);
        }
        cur_guess = cur_guess - fguess / d(f, cur_guess);

        if (cur_guess - prev_guess).abs() <= N::epsilon() {
            return Ok(cur_guess);
        }
    }
    Err("max iterations exceeded")?
}

pub fn d<N: Float, F: Fn(N) -> N>(f: F, x: N) -> N {
    let one = N::one();
    (f(x + N::epsilon()) - f(x - N::epsilon())) / ((one + one) * N::epsilon())
}

pub fn avg_color<P: Pixel, I: GenericImageView<Pixel = P>>(img: &I) -> Option<P> {
    color_avg(img.pixels().map(|(_, _, px)| px).by_ref())
}

pub fn color_avg<P: Pixel>(colors: &mut Iterator<Item = P>) -> Option<P> {
    let one = P::Subpixel::one();
    colors.next().map(|color| {
        let mut res = color; // color: Copy
        {
            let channels = res.channels_mut();
            let mut count = one;
            for c in colors {
                count = count + one;
                for (i, &channel) in c.channels().iter().enumerate() {
                    channels[i] = channels[i] + channel;
                }
            }

            for channel in channels.iter_mut() {
                *channel = *channel / count;
            }
        }
        res
    })
}

pub fn color_dist2<P: Pixel>(a: P, b: P) -> P::Subpixel {
    a.map2(&b, |a, b| {
        let diff = a - b;
        diff * diff
    }).channels()
        .iter()
        .fold(P::Subpixel::zero(), |acc, &channel| acc + channel)
}
