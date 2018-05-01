#[cfg(feature = "hyphenation")]
extern crate hyphenation;
extern crate textwrap;

#[cfg(feature = "hyphenation")]
use hyphenation::Language::English_US;
#[cfg(feature = "hyphenation")]
use textwrap::Wrapper;

#[cfg(not(feature = "hyphenation"))]
fn main() {
    println!("Please run this example as");
    println!();
    println!("  cargo run --example hyphenation --features hyphenation");
}

#[cfg(feature = "hyphenation")]
fn main() {
    let text = "textwrap: a small library for wrapping text.";
    let corpus = hyphenation::load(English_US).unwrap();
    let wrapper = Wrapper::with_splitter(18, corpus);
    println!("{}", wrapper.fill(text));
}
