extern crate regex;

pub mod tools;
pub mod first_day;
pub mod second_day;
pub mod third_day;

fn main() {
  //first_day();
  //second_day();
  third_day();
}

#[allow(dead_code)]
fn first_day() {
  println!(
    "First day first part value: {:?}",
    first_day::report_repair::compute1()
  );

  println!(
    "First day second part value: {:?}",
    first_day::report_repair::compute2()
  );
}

#[allow(dead_code)]
fn second_day() {
  println!(
    "Second day first part value: {:?}",
    second_day::password_philosophy::compute1()
  );
  println!(
    "Second day second part value: {:?}",
    second_day::password_philosophy::compute2()
  );
}

#[allow(dead_code)]
fn third_day() {
  println!(
    "Second day first part value: {:?}",
    third_day::toboggan_trajectory::compute1()
  );
  println!(
    "Second day second part value: {:?}",
    third_day::toboggan_trajectory::compute2()
  );
}
