FLAGS=RUSTFLAGS="-C target-feature=+crt-static"
BUILD=cargo build --release
CROSS=cross build --release
default:
	${FLAGS} ${BUILD}
windows:
	${BUILD} --target x86_64-pc-windows-gnu
	${BUILD} --target i686-pc-windows-gnu
# ${BUILD} --target aarch64-pc-windows-msvc
linux:
	${BUILD} --target x86_64-unknown-linux-gnu
	${CROSS} --target i686-unknown-linux-musl
	${CROSS} --target aarch64-unknown-linux-musl
# ${BUILD} --target mips-unknown-linux-gnu
# ${BUILD} --target mips64-unknown-linux-gnuabi64
# ${BUILD} --target mips64el-unknown-linux-gnuabi64
# ${BUILD} --target mipsel-unknown-linux-gnu
# ${BUILD} --target powerpc-unknown-linux-gnu
# ${BUILD} --target powerpc64-unknown-linux-gnu
# ${BUILD} --target powerpc64le-unknown-linux-gnu
freebsd:
	${CROSS} --target x86_64-unknown-freebsd
openbsd:
# ${BUILD} --target x86_64-unknown-openbsd
netbsd:
# ${BUILD} --target x86_64-unknown-netbsd
android:
	${CROSS} --target aarch64-linux-android
# ${BUILD} --target x86_64-linux-android
ios:
# ${BUILD} --target aarch64-apple-ios
# ${BUILD} --target aarch64-apple-ios-sim
darwin:
# ${BUILD} --target aarch64-apple-darwin
# ${BUILD} --target x86_64-apple-darwin
solaris:
# ${BUILD} --target x86_64-pc-solaris
illumos:
# ${BUILD} --target x86_64-unknown-illumos
dragonfly:
# ${BUILD} --target x86_64-unknown-dragonfly


all: linux windows darwin freebsd openbsd netbsd solaris aix dragonfly illumos # ${BUILD} android ios 
