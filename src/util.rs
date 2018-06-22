use image::{GenericImageView, Pixel};
use num_traits::{Float, NumCast, ToPrimitive};
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

pub fn avg_color<P: Pixel, I: GenericImageView<Pixel = P>>(img: &I) -> P {
    color_avg(img.pixels().map(|(_, _, px)| px).by_ref())
}

pub fn color_avg<P: Pixel>(colors: &mut Iterator<Item = P>) -> P {
    let (count, acc) = colors.fold((0, [0., 0., 0., 0.]), |(count, mut acc), color| {
        for (i, channel) in color.channels().iter().enumerate() {
            acc[i] += channel.to_f64().unwrap();
        }
        (count + 1, acc)
    });

    let denominator = if count == 0 { 1. } else { From::from(count) };

    P::from_channels(
        NumCast::from(acc[0] / denominator).unwrap(),
        NumCast::from(acc[1] / denominator).unwrap(),
        NumCast::from(acc[2] / denominator).unwrap(),
        NumCast::from(acc[3] / denominator).unwrap(),
    )
}

pub fn color_dist2<P: Pixel>(a: P, b: P) -> f64 {
    a.map2(&b, |a, b| {
        let diff = a - b;
        diff * diff
    }).channels()
        .iter()
        .map(|channel| channel.to_f64().unwrap())
        .sum()
}
