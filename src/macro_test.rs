#[macro_export]
macro_rules! sum {
  ($h:expr) => ($h);
  ($h:expr, $($t:expr),*) =>
      ($h + sum!($($t),*));
}

#[macro_export]
macro_rules! count {
  ($h:expr) => (1);
  ($h:expr, $($t:expr),*) =>
      (1 + count!($($t),*));
}

#[macro_export]
macro_rules! average {
  ($h:expr) => ($h);
  ($h:expr, $($t:expr),*) => (
    ($h + sum!($($t),*)) / (1 + count!($($t),*));
  );
}