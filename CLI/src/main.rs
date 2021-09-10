mod display;
mod options;
mod response;

use response::response;

fn main() {
  options::Options::print();
  // display::display();
  // match response() {
  //   Ok(data) => println!("{}", data),
  //   Err(err) => {
  //     println!("{}", err.message());
  //     main();
  //   }
  // }
}
