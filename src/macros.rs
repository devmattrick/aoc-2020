#[macro_export]
macro_rules! aoc_test {
  {
    input = $input:literal;
    $($test_part:ident = $test_output:literal);* $(;)?
  } => {
    #[cfg(test)]
    mod tests {
        use super::*;

        const RAW_INPUT: &str = $input;

        $(
          #[test]
          fn $test_part() {
            let input: &str = &RAW_INPUT
                .trim()
                .lines()
                .map(str::trim)
                .collect::<Vec<&str>>()
                            .join("\n");

            let input = generator(input);

            let solution = super::$test_part(&input);
            let output = format!("{}", solution);

            assert_eq!(output, $test_output);
          }
        )*
    }
  }
}
