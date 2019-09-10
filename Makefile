all:
	cargo rustc -- -C link-args="-lc -lgcc_s"
hisi:
	cargo rustc --target armv5te-unknown-linux-uclibc -- -C link-args="-Wl,-Bdynamic -lc -lm -lgcc -lgcc_s"
clean:
	rm -rf target
