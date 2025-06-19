use num_traits::{Float, FromPrimitive, ToPrimitive};

const SRGB_TO_XYZ: [[f64; 3]; 3] = [
  [0.41233895, 0.35762064, 0.18051042],
  [0.2126, 0.7152, 0.0722],
  [0.01932141, 0.11916382, 0.95034478],
];

const XYZ_TO_SRGB: [[f64; 3]; 3] = [
  [
    3.2413774792388685,
    -1.5376652402851851,
    -0.49885366846268053,
  ],
  [-0.9691452513005321, 1.8758853451067872, 0.04156585616912061],
  [
    0.05562093689691305,
    -0.20395524564742123,
    1.0571799111220335,
  ],
];

const WHITE_POINT_D65: [f64; 3] = [95.047, 100.0, 108.883];

#[inline]
pub fn argb_from_rgb(red: u8, green: u8, blue: u8) -> u32 {
  (255 << 24) | ((red as u32 & 255) << 16) | ((green as u32 & 255) << 8) | (blue as u32 & 255)
}

pub fn argb_from_linrgb<T>(linrgb: [f64; 3]) -> u32
where
  T: Float,
{
  let r = delinearized(linrgb[0]);
  let g = delinearized(linrgb[1]);
  let b = delinearized(linrgb[2]);
  argb_from_rgb(r, g, b)
}

/// Returns the alpha component of a color in ARGB format.
#[inline]
pub const fn alpha_from_argb(argb: u32) -> u8 {
  ((argb >> 24) & 255) as u8
}

/// Returns the red component of a color in ARGB format.
#[inline]
pub const fn red_from_argb(argb: u32) -> u8 {
  ((argb >> 16) & 255) as u8
}

/// Returns the green component of a color in ARGB format.
#[inline]
pub const fn green_from_argb(argb: u32) -> u8 {
  ((argb >> 8) & 255) as u8
}

/// Returns the blue component of a color in ARGB format.
#[inline]
pub const fn blue_from_argb(argb: u32) -> u8 {
  (argb & 255) as u8
}

#[inline]
pub fn is_opaque(argb: u32) -> bool {
  alpha_from_argb(argb) == 255
}

pub fn argb_from_xyz<T>(x: T, y: T, z: T) -> u32
where
  T: Float + FromPrimitive,
{
  let matrix = XYZ_TO_SRGB;
  let linear_r = T::from_f64(matrix[0][0]).unwrap() * x
    + T::from_f64(matrix[0][1]).unwrap() * y
    + T::from_f64(matrix[0][2]).unwrap() * z;
  let linear_g = T::from_f64(matrix[1][0]).unwrap() * x
    + T::from_f64(matrix[1][1]).unwrap() * y
    + T::from_f64(matrix[1][2]).unwrap() * z;
  let linear_b = T::from_f64(matrix[2][0]).unwrap() * x
    + T::from_f64(matrix[2][1]).unwrap() * y
    + T::from_f64(matrix[2][2]).unwrap() * z;
  let r = delinearized(linear_r);
  let g = delinearized(linear_g);
  let b = delinearized(linear_b);
  argb_from_rgb(r, g, b)
}

pub fn xyz_from_argb<T>(argb: u32) -> [T; 3]
where
  T: Float + FromPrimitive,
{
  let r: T = linearized(red_from_argb(argb));
  let g: T = linearized(green_from_argb(argb));
  let b: T = linearized(blue_from_argb(argb));
  let matrix = SRGB_TO_XYZ;
  let x = T::from_f64(matrix[0][0]).unwrap() * r
    + T::from_f64(matrix[0][1]).unwrap() * g
    + T::from_f64(matrix[0][2]).unwrap() * b;
  let y = T::from_f64(matrix[1][0]).unwrap() * r
    + T::from_f64(matrix[1][1]).unwrap() * g
    + T::from_f64(matrix[1][2]).unwrap() * b;
  let z = T::from_f64(matrix[2][0]).unwrap() * r
    + T::from_f64(matrix[2][1]).unwrap() * g
    + T::from_f64(matrix[2][2]).unwrap() * b;
  [x, y, z]
}

pub fn argb_from_lab<T>(l: T, a: T, b: T) -> u32
where
  T: Float + FromPrimitive,
{
  let white_point = WHITE_POINT_D65;
  let fy = (l + T::from_f64(16.0).unwrap()) / T::from_f64(116.0).unwrap();
  let fx = a / T::from_f64(500.0).unwrap() + fy;
  let fz = fy - b / T::from_f64(200.0).unwrap();
  let x_normalized = lab_inv_f(fx);
  let y_normalized = lab_inv_f(fy);
  let z_normalized = lab_inv_f(fz);
  let x = x_normalized * T::from_f64(white_point[0]).unwrap();
  let y = y_normalized * T::from_f64(white_point[1]).unwrap();
  let z = z_normalized * T::from_f64(white_point[2]).unwrap();
  argb_from_xyz(x, y, z)
}

pub fn lab_from_argb<T>(argb: u32) -> [T; 3]
where
  T: Float + FromPrimitive,
{
  let linear_r = linearized(red_from_argb(argb));
  let linear_g = linearized(green_from_argb(argb));
  let linear_b = linearized(blue_from_argb(argb));
  let matrix = SRGB_TO_XYZ;
  let x = T::from_f64(matrix[0][0]).unwrap() * linear_r
    + T::from_f64(matrix[0][1]).unwrap() * linear_g
    + T::from_f64(matrix[0][2]).unwrap() * linear_b;
  let y = T::from_f64(matrix[1][0]).unwrap() * linear_r
    + T::from_f64(matrix[1][1]).unwrap() * linear_g
    + T::from_f64(matrix[1][2]).unwrap() * linear_b;
  let z = T::from_f64(matrix[2][0]).unwrap() * linear_r
    + T::from_f64(matrix[2][1]).unwrap() * linear_g
    + T::from_f64(matrix[2][2]).unwrap() * linear_b;
  let white_point = WHITE_POINT_D65;
  let x_normalized = x / T::from_f64(white_point[0]).unwrap();
  let y_normalized = y / T::from_f64(white_point[1]).unwrap();
  let z_normalized = z / T::from_f64(white_point[2]).unwrap();
  let fx = lab_f(x_normalized);
  let fy = lab_f(y_normalized);
  let fz = lab_f(z_normalized);
  let l = T::from_f64(116.0).unwrap() * fy - T::from_f64(16.0).unwrap();
  let a = T::from_f64(500.0).unwrap() * (fx - fy);
  let b = T::from_f64(200.0).unwrap() * (fy - fz);
  [l, a, b]
}

pub fn argb_from_lstar<T>(lstar: T) -> u32
where
  T: Float + FromPrimitive,
{
  let y = y_from_lstar(lstar);
  let component = delinearized(y);
  argb_from_rgb(component, component, component)
}

pub fn lstar_from_argb<T>(argb: u32) -> T
where
  T: Float + FromPrimitive,
{
  let y: T = xyz_from_argb(argb)[1];
  T::from_f64(116.0).unwrap() * lab_f(y / T::from_f64(100.0).unwrap()) - T::from_f64(16.0).unwrap()
}

pub fn y_from_lstar<T>(lstar: T) -> T
where
  T: Float + FromPrimitive,
{
  T::from_f64(100.0).unwrap()
    * lab_inv_f((lstar + T::from_f64(16.0).unwrap()) / T::from_f64(116.0).unwrap())
}

pub fn lstar_from_y<T>(y: T) -> T
where
  T: Float + FromPrimitive,
{
  lab_f(y / T::from_f64(100.0).unwrap()) * T::from_f64(116.0).unwrap() - T::from_f64(16.0).unwrap()
}

pub fn linearized<T>(rgb_component: u8) -> T
where
  T: Float + PartialOrd + FromPrimitive,
{
  let normalized = T::from(rgb_component).unwrap() / T::from_f64(255.0).unwrap();
  if normalized <= T::from_f64(0.040449936).unwrap() {
    normalized / T::from_f64(12.92).unwrap() * T::from_f64(100.0).unwrap()
  } else {
    ((normalized + T::from_f64(0.055).unwrap()) / T::from_f64(1.055).unwrap())
      .powf(T::from_f64(2.4).unwrap())
      * T::from_f64(100.0).unwrap()
  }
}

pub fn delinearized<T>(rgb_component: T) -> u8
where
  T: Float + PartialOrd + FromPrimitive + ToPrimitive + Copy,
{
  let normalized = rgb_component / T::from_f64(100.0).unwrap();
  let delinearized = if normalized <= T::from_f64(0.0031308).unwrap() {
    normalized * T::from_f64(12.92).unwrap()
  } else {
    T::from_f64(1.055).unwrap()
      * normalized.powf(T::from_f64(1.0).unwrap() / T::from_f64(2.4).unwrap())
      - T::from_f64(0.055).unwrap()
  };
  ((delinearized * T::from_f64(255.0).unwrap()).round())
    .clamp(T::from_u8(0).unwrap(), T::from_u8(255).unwrap())
    .to_u8()
    .unwrap()
}

#[inline]
pub fn white_point_d65<T>() -> [T; 3]
where
  T: Float + FromPrimitive,
{
  let white_point = WHITE_POINT_D65;
  [
    T::from_f64(white_point[0]).unwrap(),
    T::from_f64(white_point[1]).unwrap(),
    T::from_f64(white_point[2]).unwrap(),
  ]
}

fn lab_f<T>(t: T) -> T
where
  T: Float + PartialOrd + FromPrimitive,
{
  let e = T::from_f64(216.0).unwrap() / T::from_f64(24389.0).unwrap();
  let kappa = T::from_f64(24389.0).unwrap() / T::from_f64(27.0).unwrap();
  if t > e {
    t.powf(T::from_f64(1.0).unwrap() / T::from_f64(3.0).unwrap())
  } else {
    (kappa * t + T::from_f64(16.0).unwrap()) / T::from_f64(116.0).unwrap()
  }
}

fn lab_inv_f<T>(ft: T) -> T
where
  T: Float + FromPrimitive,
{
  let e = T::from_f64(216.0).unwrap() / T::from_f64(24389.0).unwrap();
  let kappa = T::from_f64(24389.0).unwrap() / T::from_f64(27.0).unwrap();
  let ft3 = ft * ft * ft;
  if ft3 > e {
    ft3
  } else {
    (T::from_f64(116.0).unwrap() * ft - T::from_f64(16.0).unwrap()) / kappa
  }
}
