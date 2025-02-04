/*
  Converts f64 to string.
  This function uses ryu to enable automatic e-notations.
  However, ryu does not support rounding and trimming the xxx.0 part. 
  Current solution is definitely not the best solution but eh, I'm just too lazy to fork ryu.
*/
pub fn f64_to_string(f: f64, precision: isize) -> String {
  let mut buffer = ryu::Buffer::new();
  let s = buffer.format(f).to_string();
  let mut n: Vec<String> = s.split("e").map(|s| s.to_string()).collect();

  // Trim the .0 part
  // Since we're just working with numbers so every char is just one byte
  if n[0].ends_with(".0") {
    n[0] = n[0][0..n[0].len() - 2].to_string();
  }

  // Round the number
  if precision > -1 {
    let factor = 10_f64.powi(precision as i32);
    let n0 = n[0].parse::<f64>().unwrap();
    let n0 = (n0 * factor).round() / factor;
    n[0] = n0.to_string();
  }

  // Now with 10^ notation
  if n.len() == 2 {
    n[1] = format!("10^\"{}\"", n[1]);
  }

  // Join em back together.
  n.join(" dot ")
}