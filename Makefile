all:
	cargo rustc -- -C link-args=-lc
hisi:
	cargo rustc --target armv5te-unknown-linux-uclibc -- -C link-args="-Wl,-Bdynamic -lc -lm -lgcc"
clean:
	rm -rf target
