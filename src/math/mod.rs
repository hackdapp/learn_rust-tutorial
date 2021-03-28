use std::panic;

pub fn sum(arrays: &[u32]) -> Option<u32> {
  let result = panic::catch_unwind(|| {
    let mut total = 0;
    for item in arrays.iter() {
      total += item;
    }
    total
  });
  match result {
    Ok(res) => Some(res),
    Err(_) => None,
  }
}
