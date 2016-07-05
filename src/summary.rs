use hash::Blake2;

/// A summary contains an arbitrary blob of data and the hash of the latest
/// Event in the database which it summarizes.
#[derive(Copy, Clone, Debug, PartialEq)]
struct Summary<'a> {
    name: &'a str,
    summary: &'a [u8],
    event: Blake2,
}
