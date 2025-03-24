PKG_NAME="rfoc"
LICENSE="MIT"
AUTHORS="Kaedehito"
VERSION="1.0.0"
DESCRIPTON="file read write command"

BUILD(){
  cargo build --release
}

INSTALL(){
  cp ./target/release/rfoc .
}