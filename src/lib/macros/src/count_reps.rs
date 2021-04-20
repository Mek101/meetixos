/** # Counts the repetitions
 *
 * Counts the repetition of tokens given via argument and creates a list of
 * 1usize + 1usize + .. + 0usize for N times where N is the count of tokens
 */
#[macro_export]
macro_rules! count_reps {
    () => {
        0usize
    };
    ($_head:tt $($tail:tt)*) => {
        1usize + count_reps!($($tail)*)
    };
}
