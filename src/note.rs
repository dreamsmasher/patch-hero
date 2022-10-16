#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Note {
  /// the column in which the note plays, represented as a bitmask
  pub position: u8,
  /// the time of the note relative to the start of the track
  pub timestamp: f64,
}
