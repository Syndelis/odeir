LIBODEIR = target/libodeir.a

$(LIBODEIR):
	cargo build -Z unstable-options --out-dir target

cbindgen:
	cbindgen -o include/odeir_internal.hpp

tests:
	cargo test
	make -C tests

.PHONY: tests cbindgen
