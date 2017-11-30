# eGK xml dump
A binary that retrieves JSON from a URL via HTTP. It then extracts relevant fields and writes out some XML files.

## Configuration
The config.ini allows to configure the URL to work with. The commented out lines in it are the defaults.

# Notes
This is a very early alpha without any error handling. Only the happy path is implemented and there aren't any tests yet.

Another important thing missing is the integration with AppVeyor for continuous cross-platform delivery (main target is Windows 32bit).

## Mocked endpoint
To allow independent development there's a simple HTTP server script included. For executing it you must have Python and its flask module installed. If you want to serve the example JSON response differently you can find it in the tests folder.
