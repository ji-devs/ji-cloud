### This might be out of date... start with just npm install and cargo install as needed then refer here if you get stuck.

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

postgres (for the client library):
  1. install via the regular installer
  2. make sure that both of the postgres `bin` and `lib` dirs are on the PATH

  If not using postgres for the server (e.g. to use docker instead), make sure to manually disable it in startup so that there's no conflict with the port. On windows this conflict may not even show up as an error!

Google Cloud SDK (https://cloud.google.com/sdk/docs/quickstarts)

Google SQL Proxy (https://cloud.google.com/sql/docs/mysql/sql-proxy)
  Make sure to put it somewhere in the path and name it cloud_sql_proxy(.exe)


