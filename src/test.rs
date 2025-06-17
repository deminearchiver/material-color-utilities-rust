use num_traits::{Float, FloatConst, FromPrimitive};

#[inline]
pub(crate) fn approx_eq<T>(a: T, b: T, precision: T) -> bool
where
  T: Float + FloatConst + FromPrimitive,
{
  if a.is_infinite() {
    return a == b;
  }
  let pow = T::from_f64(10.0)
    .unwrap()
    .powf(precision + T::from_f64(1.0).unwrap());
  let delta = (a - b).abs();
  let max_delta = T::from_f64(10.0).unwrap().powf(-precision) / T::from_f64(2.0).unwrap();
  (delta * pow).round() <= max_delta * pow
}

macro_rules! assert_approx_eq {
  ($left: expr, $right: expr) => {
    assert_approx_eq!($left, $right, 2.0);
  };
  ($left: expr, $right: expr, $precision: expr) => {
    match (&$left, &$right, &$precision) {
      (left, right, precision) => {
        assert!(
          approx_eq(*left, *right, *precision),
          r#"assertion `left == right with precision` failed
  left: {:?}
  right: {:?}
  precision: {:?}"#,
          &*left,
          &*right,
          &*precision
        );
      }
    }
  };
}

pub(crate) use assert_approx_eq;
