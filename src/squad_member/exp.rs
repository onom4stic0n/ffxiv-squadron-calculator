fn calc_next_level_exp(level: u32) -> u32 {
  let (base_exp, exp_growth, scale_factor) = match level {
    1..=9 => (1_000, 100, level - 1),
    10..=19 => (2_000, 200, level - 10),
    20..=29 => (4_100, 300, level - 20),
    30..=38 => (7200, 400, level - 30),
    39..=44 => (11_000, 1_000, level - 39),
    45..=50 => (18_000, 2_000, level - 45),
    51..=54 => (46_500, 4_500, level - 51),
    55..=59 => (66_000, 6_000, level - 55),
    60 => (0, 0, 0),
    _ => panic!("Invalid level: {}", level),
  };

  base_exp + exp_growth * scale_factor
}

fn calc_total_xp_for_level(level: u32) -> u32 {
  let exp = match level {
    0 | 1 => 0,
    2..=60 => calc_next_level_exp(level - 1) + calc_total_xp_for_level(level - 1),
    _ => panic!("Invalid level: {}", level),
  };

  exp
}

fn calc_level_from_xp(exp: u32) -> u32 {
  let max_exp = 997_300;
  if exp > max_exp {
    panic!("Experience greater than max experience.")
  } else if exp == max_exp {
    return 60;
  }

  let mut level = 1;
  while calc_total_xp_for_level(level) + calc_next_level_exp(level) <= exp {
    level += 1;
  }
  level
}

#[cfg(test)]
mod calc_next_level_exp {
  use super::*;

  #[test]
  fn level_1() {
    assert_eq!(calc_next_level_exp(1), 1_000);
  }

  #[test]
  fn level_5() {
    assert_eq!(calc_next_level_exp(5), 1_400);
  }

  #[test]
  fn level_10() {
    assert_eq!(calc_next_level_exp(10), 2_000);
  }

  #[test]
  fn level_15() {
    assert_eq!(calc_next_level_exp(15), 3_000);
  }

  #[test]
  fn level_20() {
    assert_eq!(calc_next_level_exp(20), 4_100);
  }

  #[test]
  fn level_25() {
    assert_eq!(calc_next_level_exp(25), 5_600);
  }

  #[test]
  fn level_30() {
    assert_eq!(calc_next_level_exp(30), 7_200);
  }

  #[test]
  fn level_35() {
    assert_eq!(calc_next_level_exp(35), 9_200);
  }

  #[test]
  fn level_40() {
    assert_eq!(calc_next_level_exp(40), 12_000);
  }

  #[test]
  fn level_45() {
    assert_eq!(calc_next_level_exp(45), 18_000);
  }

  #[test]
  fn level_50() {
    assert_eq!(calc_next_level_exp(50), 28_000);
  }

  #[test]
  fn level_51() {
    assert_eq!(calc_next_level_exp(51), 46_500);
  }

  #[test]
  fn level_55() {
    assert_eq!(calc_next_level_exp(55), 66_000);
  }

  #[test]
  fn level_60() {
    assert_eq!(calc_next_level_exp(60), 0);
  }
}

#[cfg(test)]
mod calc_total_xp_for_level {
  use super::*;

  #[test]
  fn level_1() {
    assert_eq!(calc_total_xp_for_level(1), 0);
  }

  #[test]
  fn level_10() {
    assert_eq!(calc_total_xp_for_level(10), 12_600);
  }

  #[test]
  fn level_30() {
    assert_eq!(calc_total_xp_for_level(30), 96_100);
  }

  #[test]
  fn level_60() {
    assert_eq!(calc_total_xp_for_level(60), 997_300);
  }
}

#[cfg(test)]
mod calc_level_from_xp {
  use super::*;

  #[test]
  fn level_1_zero_xp() {
    assert_eq!(calc_level_from_xp(500), 1);
  }

  #[test]
  fn level_2_mid_xp() {
    assert_eq!(calc_level_from_xp(1500), 2);
  }

  #[test]
  fn level_10_exact_xp() {
    assert_eq!(calc_level_from_xp(12_600), 10);
  }
  #[test]
  fn level_30_one_short() {
    assert_eq!(calc_level_from_xp(103_299), 30);
  }

  #[test]
  fn level_60() {
    assert_eq!(calc_level_from_xp(997_300), 60);
  }
}
