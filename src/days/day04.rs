use std::fmt::Debug;
use regex::Regex;

fn part1(input: Vec<String>) -> u16 {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"(\w+):(.+?)\s").expect("Failed to initialize regex");
  }

  let mut valid = 0;

  for line in input {
    // Since we only need to check for the existence of the fields for part 1, we use an 8-bit bitfield to keep track
    // of which fields exist for a given passport
    let mut fields = 0x0000_0000;

    for cap in RE.captures_iter(&line) {
      let key = &cap[1];

      // Each field's existence is represented by a different bit
      match key {
        "byr" => fields |= 0x0000_0001,
        "iyr" => fields |= 0x0000_0010,
        "eyr" => fields |= 0x0000_0100,
        "hgt" => fields |= 0x0000_1000,
        "hcl" => fields |= 0x0001_0000,
        "ecl" => fields |= 0x0010_0000,
        "pid" => fields |= 0x0100_0000,
        "cid" => {}, // This field doesn't matter
        _ => panic!("Unknown key: {}", key),
      }
    }

    // The passport is "valid" if the bitfield is equal to 0x0111_1111
    if fields == 0x0111_1111 {
      valid += 1;
    }
  }

  valid
}

const VALID_ECLS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

#[derive(Debug)]
struct Passport {
  byr: Option<String>,
  iyr: Option<String>,
  eyr: Option<String>,
  hgt: Option<String>,
  hcl: Option<String>,
  ecl: Option<String>,
  pid: Option<String>,
  cid: Option<String>,
}

impl Passport {
  fn new() -> Passport {
    Passport {
      byr: Option::None,
      iyr: Option::None,
      eyr: Option::None,
      hgt: Option::None,
      hcl: Option::None,
      ecl: Option::None,
      pid: Option::None,
      cid: Option::None,
    }
  }

  fn is_valid(&self) -> bool {
    self.valid_byr() && self.valid_iyr() && self.valid_eyr() && self.valid_hgt() && self.valid_hcl() &&
    self.valid_ecl() && self.valid_pid() && self.valid_cid()
  }

  fn valid_byr(&self) -> bool {
    match &self.byr {
      Some(byr) => {
        let byr = byr.parse::<u16>();

        match byr {
          Ok(byr) => byr >= 1920 && byr <= 2002,
          Err(_) => false,
        }
      },
      None => false,
    }
  }

  fn valid_iyr(&self) -> bool {
    match &self.iyr {
      Some(iyr) => {
        let iyr = iyr.parse::<u16>();

        match iyr {
          Ok(iyr) => iyr >= 2010 && iyr <= 2020,
          Err(_) => false,
        }
      },
      None => false,
    }
  }

  fn valid_eyr(&self) -> bool {
    match &self.eyr {
      Some(eyr) => {
        let eyr = eyr.parse::<u16>();

        match eyr {
          Ok(eyr) => eyr >= 2020 && eyr <= 2030,
          Err(_) => false,
        }
      },
      None => false,
    }
  }

  fn valid_hgt(&self) -> bool {
    match &self.hgt {
      Some(hgt) => {
        let value = (&hgt[..hgt.len() - 2]).parse::<u16>();

        match value {
          Ok(value) => {
            match &hgt[hgt.len() - 2..] {
              "cm" => value >= 150 && value <= 193,
              "in" => value >= 59 && value <= 76,
              _ => false,
            }
          },
          Err(_) => false,
        }
      },
      None => false,
    }
  }

  fn valid_hcl(&self) -> bool {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"#[0-9a-f]{6}").expect("Failed to initialize regex");
    }

    match &self.hcl {
      Some(hcl) => RE.is_match(hcl.as_str()),
      None => false,
    }
  }

  fn valid_ecl(&self) -> bool {
    match &self.ecl {
      Some(ecl) => VALID_ECLS.contains(&ecl.as_str()),
      None => false,
    }
  }

  fn valid_pid(&self) -> bool {
    match &self.pid {
      Some(pid) => pid.len() == 9 && pid.parse::<u32>().is_ok(),
      None => false,
    }
  }

  fn valid_cid(&self) -> bool {
    true
  }
}

fn part2 (input: Vec<String>) -> u16 {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"(\w+):(.+?)\s").expect("Failed to initialize regex");
  }

  let mut valid = 0;

  for line in input {
    let mut passport = Passport::new();

    for cap in RE.captures_iter(&line) {
      let key = &cap[1];
      let value = &cap[2];

      match key {
        "byr" => passport.byr = Option::from(value.to_owned()),
        "iyr" => passport.iyr = Option::from(value.to_owned()),
        "eyr" => passport.eyr = Option::from(value.to_owned()),
        "hgt" => passport.hgt = Option::from(value.to_owned()),
        "hcl" => passport.hcl = Option::from(value.to_owned()),
        "ecl" => passport.ecl = Option::from(value.to_owned()),
        "pid" => passport.pid = Option::from(value.to_owned()),
        "cid" => passport.cid = Option::from(value.to_owned()),
        _ => panic!("Unknown key: {}", key),
      }
    }

    if passport.is_valid() {
      valid += 1;
    }
  }

  valid
}

#[cfg(test)]
mod tests {
  use std::fs;
  use super::*;

  fn read_input() -> Vec<String> {
    let input = fs::read_to_string("input/04.txt").expect("Failed to read file");
    let input: Vec<&str> = input.lines().collect();


    // Separate passport records into discrete entries
    let mut passports = Vec::new();
    let mut passport = String::new();
    for line in input {
      if line.is_empty() {
        passports.push(passport);
        passport = String::new();
        continue;
      }

      passport.push_str(line);
      // Add a space between lines. This will leave a trailing space in our strings, but makes the regex simpler
      passport.push(' ');
    }

    // Push final passport
    passports.push(passport);

    passports
  }

  #[test]
  fn test_day04_part1() {
    let answer = part1(read_input());

    println!("Part 1 Answer: {}", answer);
  }


  #[test]
  fn test_day04_part2() {
    let answer = part2(read_input());

    println!("Part 2 Answer: {}", answer);
  }
}
