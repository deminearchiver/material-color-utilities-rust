use std::f64;

use num_traits::{Float, FloatConst, FromPrimitive, ToPrimitive};

use crate::{
  hct::{Cam16, ViewingConditions},
  utils,
};

const SCALED_DISCOUNT_FROM_LINRGB: [[f64; 3]; 3] = [
  [
    0.001200833568784504,
    0.002389694492170889,
    0.0002795742885861124,
  ],
  [
    0.0005891086651375999,
    0.0029785502573438758,
    0.0003270666104008398,
  ],
  [
    0.00010146692491640572,
    0.0005364214359186694,
    0.0032979401770712076,
  ],
];

const LINRGB_FROM_SCALED_DISCOUNT: [[f64; 3]; 3] = [
  [1373.2198709594231, -1100.4251190754821, -7.278681089101213],
  [-271.815969077903, 559.6580465940733, -32.46047482791194],
  [1.9622899599665666, -57.173814538844006, 308.7233197812385],
];

const Y_FROM_LINRGB: [f64; 3] = [0.2126, 0.7152, 0.0722];

const CRITICAL_PLANES: [f64; 255] = [
  0.015176349177441876,
  0.045529047532325624,
  0.07588174588720938,
  0.10623444424209313,
  0.13658714259697685,
  0.16693984095186062,
  0.19729253930674434,
  0.2276452376616281,
  0.2579979360165119,
  0.28835063437139563,
  0.3188300904430532,
  0.350925934958123,
  0.3848314933096426,
  0.42057480301049466,
  0.458183274052838,
  0.4976837250274023,
  0.5391024159806381,
  0.5824650784040898,
  0.6277969426914107,
  0.6751227633498623,
  0.7244668422128921,
  0.775853049866786,
  0.829304845476233,
  0.8848452951698498,
  0.942497089126609,
  1.0022825574869039,
  1.0642236851973577,
  1.1283421258858297,
  1.1946592148522128,
  1.2631959812511864,
  1.3339731595349034,
  1.407011200216447,
  1.4823302800086415,
  1.5599503113873272,
  1.6398909516233677,
  1.7221716113234105,
  1.8068114625156377,
  1.8938294463134073,
  1.9832442801866852,
  2.075074464868551,
  2.1693382909216234,
  2.2660538449872063,
  2.36523901573795,
  2.4669114995532007,
  2.5710888059345764,
  2.6777882626779785,
  2.7870270208169257,
  2.898822059350997,
  3.0131901897720907,
  3.1301480604002863,
  3.2497121605402226,
  3.3718988244681087,
  3.4967242352587946,
  3.624204428461639,
  3.754355295633311,
  3.887192587735158,
  4.022731918402185,
  4.160988767090289,
  4.301978482107941,
  4.445716283538092,
  4.592217266055746,
  4.741496401646282,
  4.893568542229298,
  5.048448422192488,
  5.20615066083972,
  5.3666897647573375,
  5.5300801301023865,
  5.696336044816294,
  5.865471690767354,
  6.037501145825082,
  6.212438385869475,
  6.390297286737924,
  6.571091626112461,
  6.7548350853498045,
  6.941541251256611,
  7.131223617812143,
  7.323895587840543,
  7.5195704746346665,
  7.7182615035334345,
  7.919981813454504,
  8.124744458384042,
  8.332562408825165,
  8.543448553206703,
  8.757415699253682,
  8.974476575321063,
  9.194643831691977,
  9.417930041841839,
  9.644347703669503,
  9.873909240696694,
  10.106627003236781,
  10.342513269534024,
  10.58158024687427,
  10.8238400726681,
  11.069304815507364,
  11.317986476196008,
  11.569896988756009,
  11.825048221409341,
  12.083451977536606,
  12.345119996613247,
  12.610063955123938,
  12.878295467455942,
  13.149826086772048,
  13.42466730586372,
  13.702830557985108,
  13.984327217668513,
  14.269168601521828,
  14.55736596900856,
  14.848930523210871,
  15.143873411576273,
  15.44220572664832,
  15.743938506781891,
  16.04908273684337,
  16.35764934889634,
  16.66964922287304,
  16.985093187232053,
  17.30399201960269,
  17.62635644741625,
  17.95219714852476,
  18.281524751807332,
  18.614349837764564,
  18.95068293910138,
  19.290534541298456,
  19.633915083172692,
  19.98083495742689,
  20.331304511189067,
  20.685334046541502,
  21.042933821039977,
  21.404114048223256,
  21.76888489811322,
  22.137256497705877,
  22.50923893145328,
  22.884842241736916,
  23.264076429332462,
  23.6469514538663,
  24.033477234264016,
  24.42366364919083,
  24.817520537484558,
  25.21505769858089,
  25.61628489293138,
  26.021211842414342,
  26.429848230738664,
  26.842203703840827,
  27.258287870275353,
  27.678110301598522,
  28.10168053274597,
  28.529008062403893,
  28.96010235337422,
  29.39497283293396,
  29.83362889318845,
  30.276079891419332,
  30.722335150426627,
  31.172403958865512,
  31.62629557157785,
  32.08401920991837,
  32.54558406207592,
  33.010999283389665,
  33.4802739966603,
  33.953417292456834,
  34.430438229418264,
  34.911345834551085,
  35.39614910352207,
  35.88485700094671,
  36.37747846067349,
  36.87402238606382,
  37.37449765026789,
  37.87891309649659,
  38.38727753828926,
  38.89959975977785,
  39.41588851594697,
  39.93615253289054,
  40.460400508064545,
  40.98864111053629,
  41.520882981230194,
  42.05713473317016,
  42.597404951718396,
  43.141702194811224,
  43.6900349931913,
  44.24241185063697,
  44.798841244188324,
  45.35933162437017,
  45.92389141541209,
  46.49252901546552,
  47.065252796817916,
  47.64207110610409,
  48.22299226451468,
  48.808024568002054,
  49.3971762874833,
  49.9904556690408,
  50.587870934119984,
  51.189430279724725,
  51.79514187861014,
  52.40501387947288,
  53.0190544071392,
  53.637271562750364,
  54.259673423945976,
  54.88626804504493,
  55.517063457223934,
  56.15206766869424,
  56.79128866487574,
  57.43473440856916,
  58.08241284012621,
  58.734331877617365,
  59.39049941699807,
  60.05092333227251,
  60.715611475655585,
  61.38457167773311,
  62.057811747619894,
  62.7353394731159,
  63.417162620860914,
  64.10328893648692,
  64.79372614476921,
  65.48848194977529,
  66.18756403501224,
  66.89098006357258,
  67.59873767827808,
  68.31084450182222,
  69.02730813691093,
  69.74813616640164,
  70.47333615344107,
  71.20291564160104,
  71.93688215501312,
  72.67524319850172,
  73.41800625771542,
  74.16517879925733,
  74.9167682708136,
  75.67278210128072,
  76.43322770089146,
  77.1981124613393,
  77.96744375590167,
  78.74122893956174,
  79.51947534912904,
  80.30219030335869,
  81.08938110306934,
  81.88105503125999,
  82.67721935322541,
  83.4778813166706,
  84.28304815182372,
  85.09272707154808,
  85.90692527145302,
  86.72564993000343,
  87.54890820862819,
  88.3767072518277,
  89.2090541872801,
  90.04595612594655,
  90.88742016217518,
  91.73345337380438,
  92.58406282226491,
  93.43925555268066,
  94.29903859396902,
  95.16341895893969,
  96.03240364439274,
  96.9059996312159,
  97.78421388448044,
  98.6670533535366,
  99.55452497210776,
];

/// Sanitizes a small enough angle in radians.
fn sanitize_radians<T>(angle: T) -> T
where
  T: Float + FloatConst + FromPrimitive,
{
  (angle + T::PI() * T::from_f64(8.0).unwrap()) % (T::PI() * T::from_f64(2.0).unwrap())
}

/// Delinearizes an RGB component, returning a floating-point number.
fn true_delinearized<T>(rgb_component: T) -> T
where
  T: Float + FromPrimitive,
{
  let normalized = rgb_component / T::from_f64(100.0).unwrap();
  let delinearized = if normalized <= T::from_f64(0.0031308).unwrap() {
    normalized * T::from_f64(12.92).unwrap()
  } else {
    T::from_f64(1.055).unwrap()
      * normalized.powf(T::from_f64(1.0).unwrap() / T::from_f64(2.4).unwrap())
      - T::from_f64(0.055).unwrap()
  };
  delinearized * T::from_f64(255.0).unwrap()
}

fn chromatic_adaptation<T>(component: T) -> T
where
  T: Float + FromPrimitive,
{
  let af = component.abs().powf(T::from_f64(0.42).unwrap());
  component.signum() * T::from_f64(400.0).unwrap() * af / (af + T::from_f64(27.13).unwrap())
}

/// Returns the hue of a linear RGB color in CAM16.
fn hue_of(linrgb: [f64; 3]) -> f64 {
  let scaled_discount = utils::math::matrix_multiply(&linrgb, &SCALED_DISCOUNT_FROM_LINRGB);
  let r_a = chromatic_adaptation(scaled_discount[0]);
  let g_a = chromatic_adaptation(scaled_discount[1]);
  let b_a = chromatic_adaptation(scaled_discount[2]);
  // redness-greenness
  let a = (11.0 * r_a + -12.0 * g_a + b_a) / 11.0;
  // yellowness-blueness
  let b = (r_a + g_a - 2.0 * b_a) / 9.0;
  f64::atan2(b, a)
}

fn are_in_cyclic_order<T>(a: T, b: T, c: T) -> bool
where
  T: Float + FloatConst + FromPrimitive,
{
  let delta_a_b = sanitize_radians(b - a);
  let delta_a_c = sanitize_radians(c - a);
  delta_a_b < delta_a_c
}

/// Solves the lerp equation.
fn intercept<T>(source: T, mid: T, target: T) -> T
where
  T: Float,
{
  (mid - source) / (target - source)
}

fn lerp_point<T>(source: [T; 3], t: T, target: [T; 3]) -> [T; 3]
where
  T: Float,
{
  [
    source[0] + (target[0] - source[0]) * t,
    source[1] + (target[1] - source[1]) * t,
    source[2] + (target[2] - source[2]) * t,
  ]
}

/// Intersects a segment with a plane.
fn set_coordinate<T>(source: [T; 3], coordinate: T, target: [T; 3], axis: usize) -> [T; 3]
where
  T: Float,
{
  let t = intercept(source[axis], coordinate, target[axis]);
  lerp_point(source, t, target)
}

fn is_bounded<T>(x: T) -> bool
where
  T: Float + FromPrimitive,
{
  T::zero() <= x && x <= T::from_f64(100.0).unwrap()
}

/// Returns the nth possible vertex of the polygonal intersection.
fn nth_vertex(y: f64, n: usize) -> [f64; 3] {
  let k_r = Y_FROM_LINRGB[0];
  let k_g = Y_FROM_LINRGB[1];
  let k_b = Y_FROM_LINRGB[2];
  let coord_a = if n % 4 <= 1 { 0.0 } else { 100.0 };
  let coord_b = if n % 2 == 0 { 0.0 } else { 100.0 };
  if n < 4 {
    let g = coord_a;
    let b = coord_b;
    let r = (y - g * k_g - b * k_b) / k_r;
    if is_bounded(r) {
      [r, g, b]
    } else {
      [-1.0, -1.0, -1.0]
    }
  } else if n < 8 {
    let b = coord_a;
    let r = coord_b;
    let g = (y - r * k_r - b * k_b) / k_g;
    if is_bounded(g) {
      [r, g, b]
    } else {
      [-1.0, -1.0, -1.0]
    }
  } else {
    let r = coord_a;
    let g = coord_b;
    let b = (y - r * k_r - g * k_g) / k_b;
    if is_bounded(b) {
      [r, g, b]
    } else {
      [-1.0, -1.0, -1.0]
    }
  }
}

/// Finds the segment containing the desired color.
fn bisect_to_segment(y: f64, target_hue: f64) -> [[f64; 3]; 2] {
  let mut left = [-1.0, -1.0, -1.0];
  let mut right = left;
  let mut left_hue = 0.0;
  let mut right_hue = 0.0;
  let mut initialized = false;
  let mut uncut = true;
  for n in 0..12 {
    let mid = nth_vertex(y, n);
    if mid[0] < 0.0 {
      continue;
    }
    let mid_hue = hue_of(mid);
    if !initialized {
      left = mid;
      right = mid;
      left_hue = mid_hue;
      right_hue = mid_hue;
      initialized = true;
      continue;
    }
    if uncut || are_in_cyclic_order(left_hue, mid_hue, right_hue) {
      uncut = false;
      if are_in_cyclic_order(left_hue, target_hue, mid_hue) {
        right = mid;
        right_hue = mid_hue;
      } else {
        left = mid;
        left_hue = mid_hue;
      }
    }
  }
  [left, right]
}

fn midpoint<T>(a: [T; 3], b: [T; 3]) -> [T; 3]
where
  T: Float + FromPrimitive,
{
  [
    (a[0] + b[0]) / T::from_f64(2.0).unwrap(),
    (a[1] + b[1]) / T::from_f64(2.0).unwrap(),
    (a[2] + b[2]) / T::from_f64(2.0).unwrap(),
  ]
}

fn critical_plane_below<T>(x: T) -> isize
where
  T: Float + FromPrimitive + ToPrimitive,
{
  (x - T::from_f64(0.5).unwrap()).floor().to_isize().unwrap()
}

fn critical_plane_above<T>(x: T) -> isize
where
  T: Float + FromPrimitive + ToPrimitive,
{
  (x - T::from_f64(0.5).unwrap()).ceil().to_isize().unwrap()
}

/// Finds a color with the given Y and hue on the boundary of the cube.
fn bisect_to_limit(y: f64, target_hue: f64) -> [f64; 3] {
  let segment = bisect_to_segment(y, target_hue);
  let mut left = segment[0];
  let mut left_hue = hue_of(left);
  let mut right = segment[1];
  for axis in 0..=2 {
    if left[axis] != right[axis] {
      let (mut l_plane, mut r_plane) = if left[axis] < right[axis] {
        (
          critical_plane_below(true_delinearized(left[axis])),
          critical_plane_above(true_delinearized(right[axis])),
        )
      } else {
        (
          critical_plane_above(true_delinearized(left[axis])),
          critical_plane_below(true_delinearized(right[axis])),
        )
      };
      for _ in 0..8 {
        if (r_plane - l_plane).abs() <= 1 {
          break;
        } else {
          let m_plane = ((l_plane + r_plane) as f64 / 2.0).floor() as isize;
          let mid_plane_coordinate = CRITICAL_PLANES[m_plane as usize];
          let mid = set_coordinate(left, mid_plane_coordinate, right, axis);
          let mid_hue = hue_of(mid);
          if are_in_cyclic_order(left_hue, target_hue, mid_hue) {
            right = mid;
            r_plane = m_plane;
          } else {
            left = mid;
            left_hue = mid_hue;
            l_plane = m_plane;
          }
        }
      }
    }
  }
  midpoint(left, right)
}

fn inverse_chromatic_adaptation<T>(adapted: T) -> T
where
  T: Float + FromPrimitive,
{
  let adapted_abs = adapted.abs();
  let base = T::max(
    T::from_f64(0.0).unwrap(),
    T::from_f64(27.13).unwrap() * adapted_abs / (T::from_f64(400.0).unwrap() - adapted_abs),
  );
  adapted.signum() * base.powf(T::from_f64(1.0).unwrap() / T::from_f64(0.42).unwrap())
}

/// Finds a color with the given hue, chroma, and Y.
fn find_result_by_j(hue_radians: f64, chroma: f64, y: f64) -> u32 {
  // Initial estimate of j.
  let mut j = y.sqrt() * 11.0;
  // ===========================================================
  // Operations inlined from Cam16 to avoid repeated calculation
  // ===========================================================
  let viewing_conditions = ViewingConditions::default();
  let t_inner_coeff = 1.0 / f64::powf(1.64 - f64::powf(0.29, viewing_conditions.n()), 0.73);
  let e_hue = 0.25 * (f64::cos(hue_radians + 2.0) + 3.8);
  let p1 = e_hue * (50000.0 / 13.0) * viewing_conditions.nc() * viewing_conditions.ncb();
  let h_sin = hue_radians.sin();
  let h_cos = hue_radians.cos();

  for iteration_round in 0..5 {
    // ===========================================================
    // Operations inlined from Cam16 to avoid repeated calculation
    // ===========================================================
    let j_normalized = j / 100.0;
    let alpha = if chroma == 0.0 || j == 0.0 {
      0.0
    } else {
      chroma / j_normalized.sqrt()
    };
    let t = f64::powf(alpha * t_inner_coeff, 1.0 / 0.9);
    let ac = viewing_conditions.aw()
      * f64::powf(
        j_normalized,
        1.0 / viewing_conditions.c() / viewing_conditions.z(),
      );
    let p2 = ac / viewing_conditions.nbb();
    let gamma = 23.0 * (p2 + 0.305) * t / (23.0 * p1 + 11.0 * t * h_cos + 108.0 * t * h_sin);
    let a = gamma * h_cos;
    let b = gamma * h_sin;
    let r_a = (460.0 * p2 + 451.0 * a + 288.0 * b) / 1403.0;
    let g_a = (460.0 * p2 - 891.0 * a - 261.0 * b) / 1403.0;
    let b_a = (460.0 * p2 - 220.0 * a - 6300.0 * b) / 1403.0;
    let r_c_scaled = inverse_chromatic_adaptation(r_a);
    let g_c_scaled = inverse_chromatic_adaptation(g_a);
    let b_c_scaled = inverse_chromatic_adaptation(b_a);
    let linrgb = utils::math::matrix_multiply(
      &[r_c_scaled, g_c_scaled, b_c_scaled],
      &LINRGB_FROM_SCALED_DISCOUNT,
    );
    // ===========================================================
    // Operations inlined from Cam16 to avoid repeated calculation
    // ===========================================================
    if linrgb[0] < 0.0 || linrgb[1] < 0.0 || linrgb[2] < 0.0 {
      return 0;
    }
    let k_r = Y_FROM_LINRGB[0];
    let k_g = Y_FROM_LINRGB[1];
    let k_b = Y_FROM_LINRGB[2];
    let fnj = k_r * linrgb[0] + k_g * linrgb[1] + k_b * linrgb[2];
    if fnj <= 0.0 {
      return 0;
    }
    if iteration_round == 4 || (fnj - y).abs() < 0.002 {
      if linrgb[0] > 100.01 || linrgb[1] > 100.01 || linrgb[2] > 100.01 {
        return 0;
      }
      return utils::color::argb_from_linrgb::<f64>(linrgb);
    }
    // Iterates with Newton method,
    // Using 2 * fn(j) / j as the approximation of fn'(j)
    j = j - (fnj - y) * j / (2.0 * fnj);
  }
  0
}

/// Finds an sRGB color with the given hue, chroma, and L*, if possible.
pub fn solve_to_int(hue_degrees: f64, chroma: f64, lstar: f64) -> u32 {
  if chroma < 0.0001 || !(0.0001..=99.9999).contains(&lstar) {
    return utils::color::argb_from_lstar(lstar);
  }
  let hue_degrees = utils::math::sanitize_degrees(hue_degrees);
  let hue_radians = hue_degrees / 180.0 * f64::consts::PI;
  let y = utils::color::y_from_lstar(lstar);
  let exact_answer = find_result_by_j(hue_radians, chroma, y);
  if exact_answer != 0 {
    return exact_answer;
  }
  let linrgb = bisect_to_limit(y, hue_radians);
  utils::color::argb_from_linrgb::<f64>(linrgb)
}

/// Finds an sRGB color with the given hue, chroma, and L*, if possible.
#[allow(dead_code)]
pub fn solve_to_cam(hue_degrees: f64, chroma: f64, lstar: f64) -> Cam16 {
  Cam16::from_int(solve_to_int(hue_degrees, chroma, lstar))
}
