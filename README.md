# eGK xml dump
A binary that retrieves JSON from a URL via HTTP. It then extracts relevant fields and writes out some XML files.

## Releases

* [0.1.0](https://httpd.ehex.de/internal/egk_xml_dump-0.1.0.zip) Initial release

## Configuration
The config.ini allows to configure the URL to work with. The commented out lines in it are the defaults.

## Local build environment (for Windows as target platform)
* install virtualbox + vagrant
* `vagrant up`(in project root)
* in the started VM install
  * Rust toolchain ([rustup.rs](https://rustup.rs/))
  * Visual C++ 2015
  * Virtualbox guest additions
    * you need to add a CD drive to the VM before you can inject them
  * `net use x: \\vboxsrv\vagrant`
  * in `x:` execute `cargo build --release`
  * archive the binary with a `config.ini`
  * deploy the archive to the internal httpd (betazed:/opt/httpd/htdocs/internal/)

# TODOs
* tests
* build automation

# Mocked endpoint
To allow independent development there's a simple HTTP server script included. For executing it you must have Python and its flask module installed. If you want to serve the example JSON response differently you can find it in the tests folder.
