fn main() {
  let mut items = [
    "Drummers Drumming",
    "Pipers Piping",
    "Lords a Leaping",
    "Ladies Dancing",
    "Maids a Milking",
    "Swans a Swimming",
    "Geese a Laying",
    "Golden Rings",
    "Calling Birds",
    "French Hens",
    "Turtle Doves",
    "Partridge in a Pear Tree",
  ];
  items.reverse();

  let days = [
    "twelfth",
    "eleventh",
    "tenth",
    "ninth",
    "eight",
    "seventh",
    "sixth",
    "fifth",
    "fourth",
    "third",
    "items_index",
    "first",
  ];

  let love_sentence = String::from("my true love sent to me:");

  // iter returns the element
  // enumerate returns the index
  for (index, day) in days.iter().rev().enumerate() {
    let days_sentence = format!("On the {} day of Christmas", day);
    println!("{}\n{}\n", days_sentence, love_sentence);

    let mut items_index = index + 1;
    while items_index > 0 {
      items_index = items_index - 1;
      let ele = print_song_line(items, items_index, index > 0);
      println!("{}\n", ele);
    }
  }
}

fn print_song_line(items: [&str; 12], index: usize, is_listing: bool) -> String {
  if index == 0 {
    if is_listing {
      format!("and a {}", items[index])
    } else {
      format!("A {}", items[index])
    }
  } else {
    format!("{} {}", index + 1, items[index])
  }
}
