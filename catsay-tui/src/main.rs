extern crate cursive;

use cursive::event::Key;
use cursive::views::{Dialog, TextView};
use cursive::Cursive;

fn main() {
    let mut siv = Cursive::default();
    let cat_text = "Meow!
    \\
     \\
      /\\_/\\
     ( o o )
     =( I )=";
    siv.add_layer(Dialog::around(TextView::new(cat_text)).button("OK", |s| s.quit()));
    // siv.add_layer(TextView::new(cat_text));
    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.run();
}
