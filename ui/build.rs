// public/assets/app.css is generated from the numbered files in css/ - never edit it by hand.
fn main() {
    ci_utils::css::CssCompiler::new("./css")
        .add_file("01-tokens.css")
        .add_file("02-base.css")
        .add_file("03-shell.css")
        .add_file("04-atoms.css")
        .add_file("05-tree.css")
        .add_file("06-details.css")
        .compile("./public/assets/app.css");
}
