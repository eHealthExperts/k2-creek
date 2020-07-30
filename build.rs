#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let res = winres::WindowsResource::new();
    let _ = res.compile().expect("Failed to run resource compiler!");
}

#[cfg(unix)]
fn main() {}
