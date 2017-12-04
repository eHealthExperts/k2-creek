# eGK xml dump
A binary that retrieves JSON from a URL via HTTP. It then extracts relevant fields and writes out some XML files.

## Releases

* [0.1.0](https://httpd.ehex.de/internal/egk_xml_dump-0.1.0.zip) Initial release

## Configuration
The `config.ini` allows to configure the URL to work with. The commented out lines in it are the defaults.

## Local build environment (for Windows as target platform)
* install virtualbox + vagrant
* `vagrant up` (in `build_vm` folder)
  * it's in a separate folder because files next to it get auto-mounted into the VM
  * it appears desirable to have autoamtic access to the project files, but it causes trouble with the rust toolchain inside the VM, so just do a separate clone in the build_vm folder and work with that
* in the started VM
  * install Rust toolchain ([rustup.rs](https://rustup.rs/))
  * install [Visual C++ Build Tools 2015](http://landinghub.visualstudio.com/visual-cpp-build-tools)
  * install Virtualbox guest additions
     * you need to add a CD drive to the VM before you can inject them
  * `net use x: \\vboxsrv\vagrant` or access via "Network" and map it to a drive
  * in the mounted project folder execute `cargo build --release`
  * archive the binary together with a `config.ini`
  * deploy the archive to the internal httpd (betazed:/opt/httpd/htdocs/internal/)

# TODOs
* tests
* build automation

# Mocked endpoint
To allow independent development there's a simple HTTP server script included. For executing it you must have Python 3 and its `flask` module installed. If you want to serve the example JSON response differently you can find it in the tests folder.

## Running the mock on Windows
If you want to run the `k2_mock.py` on Windows, you need to install [Python](https://www.python.org/downloads/release) (make sure to check the checkbox to add it to the env vars) and run `pip install flask` on the command line, just as the UNIX users do.
