//! A crate declaring data for spinners (name, frames and interval).
//!
//! The structs for this crate are derived by parsing Sindre Sorhus' [cli-spinner](https://github.com/sindresorhus/cli-spinners) npm package.

include!(concat!(env!("OUT_DIR"), "/spinners.rs"));

/// Data related to a spinner.
///
/// Each spinner consists of a number of frames and an interval. The interval is
/// used for animation and should be the amount of milliseconds between each
/// frame.
///
/// Frames are cycled through, meaning that if you reach the last element inside the frames' array, you should wrap around and start again at the beginning.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpinnerData<'a> {
    pub frames: &'a [&'a str],
    pub interval: u64,
}
