#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let res = winres::WindowsResource::new();
    res.compile().expect("Failed to run resource compiler!");
}

#[cfg(unix)]
fn main() {}
