use num_traits::{FromPrimitive, Num, Signed};

pub fn lerp<T>(start: T, stop: T, amount: T) -> T
where
  T: Num + Copy,
{
  (T::one() - amount) * start + amount * stop
}

pub fn sanitize_degrees<T>(mut degrees: T) -> T
where
  T: Num + PartialOrd + FromPrimitive,
{
  let zero = T::from_f64(0.0).unwrap();
  degrees = degrees % T::from_f64(360.0).unwrap();
  if degrees < zero {
    degrees = degrees + T::from_f64(360.0).unwrap();
  }
  degrees
}

pub fn rotation_direction<T>(from: T, to: T) -> T
where
  T: Num + PartialOrd + FromPrimitive,
{
  let increasing_difference = sanitize_degrees(to - from);
  if increasing_difference <= T::from_f64(180.0).unwrap() {
    T::from_f64(1.0).unwrap()
  } else {
    T::from_f64(-1.0).unwrap()
  }
}

pub fn difference_degrees<T>(a: T, b: T) -> T
where
  T: Num + Signed + FromPrimitive,
{
  T::from_f64(180.0).unwrap()
    - ((a - b).abs() - T::from_f64(180.0).unwrap()).abs()
}

pub fn matrix_multiply<T>(row: &[T; 3], matrix: &[[T; 3]; 3]) -> [T; 3]
where
  T: Num + Copy,
{
  let a = row[0] * matrix[0][0] + row[1] * matrix[0][1] + row[2] * matrix[0][2];
  let b = row[0] * matrix[1][0] + row[1] * matrix[1][1] + row[2] * matrix[1][2];
  let c = row[0] * matrix[2][0] + row[1] * matrix[2][1] + row[2] * matrix[2][2];
  [a, b, c]
}
