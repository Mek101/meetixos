/*! Miscellaneous helpers functions */

/**
 * Identity function to ease moving of references.
 *
 * By default, references are re-borrowed instead of moved (equivalent to
 * `&mut *reference`). This function forces a move.
 *
 * for more information, see section “id Forces References To Move” in:
 * https://bluss.github.io/rust/fun/2015/10/11/stuff-the-identity-function-does/
 */
pub fn force_move<T>(ref_value: T) -> T {
    ref_value
}
