/*
  Fixes obviously wrong floating point issues (e.g. 0.1 + 0.2 = 0.30000000000000004)
*/
pub fn clean_f64_trailing_pattern(s: &str) -> String {
  let re = regex::Regex::new(r"^(\d+)\.(\d*?)0{5,}[1-9]$").unwrap();
  
  if let Some(captures) = re.captures(s) {
    // Reconstruct the number without the unwanted trailing part
    format!("{}.{}", &captures[1], &captures[2])
  } else {
    s.to_string()
  }
}