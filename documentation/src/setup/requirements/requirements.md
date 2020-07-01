cargo install:
  * systemfd
  * watchexec
  * cargo-make
  * cargo-watch

wasm-pack (https://rustwasm.github.io/wasm-pack/installer/)

openssl:
  1. visit https://slproweb.com/products/Win32OpenSSL.html (yes, the site says win32 but it has win64 msi)
  2. after installing, add `C:\Program Files\OpenSSL-Win64\bin` to path
  3. add `C:\Program Files\OpenSSL-Win64` to `OPENSSL_DIR` env var

postgres (for the library):
  1. install via the regular installer
  2. make sure that both of the postgres `bin` and `lib` dirs are on the PATH
