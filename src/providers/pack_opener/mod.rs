use cursive::Cursive;

pub fn handler(screen: &mut Cursive) {
    screen.pop_layer();
    screen.add_layer(cursive::views::Dialog::text("Hello, world!"));
}