use image::Pixel;
use num_traits::{Float, One, Zero};
use std::error::Error;

pub fn newtons_method<N: Float, F: Fn(N) -> N>(f: &F, guess: N) -> Result<N, Box<Error>> {
    let mut curGuess = guess;
    for _ in 0.. {
        let prevGuess = curGuess;

        let fguess = f(curGuess);
        if fguess.abs() <= N::epsilon() {
            // found a root!
            return Ok(curGuess);
        }
        curGuess = curGuess - fguess / d(f, curGuess);

        if (curGuess - prevGuess).abs() <= N::epsilon() {
            return Ok(curGuess);
        }
    }
    Err("max iterations exceeded")?
}

pub fn d<N: Float, F: Fn(N) -> N>(f: F, x: N) -> N {
    let one = N::one();
    (f(x + N::epsilon()) - f(x - N::epsilon())) / ((one + one) * N::epsilon())
}

pub fn color_avg<P: Pixel>(colors: &mut Iterator<Item = P>) -> Option<P> {
    let one = P::Subpixel::one();
    colors.next().map(|color| {
        let mut res = color.clone();
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
