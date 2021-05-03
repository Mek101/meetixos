#! Userland Applications Builder Script
#!
#! This Makefile is responsible to build/install/clean/doc the userland applications

include ../../config.make

#
# -- -- -- -- -- -- -- -- -- -- -- -- -- Projects -- -- -- -- -- -- -- -- -- -- -- -- --
#

APPS := $(dir $(basename $(wildcard */Cargo.toml)))

#
# -- -- -- -- -- -- -- -- -- -- -- -- -- Make Targets -- -- -- -- -- -- -- -- -- -- -- --
#

build:
	$(V) echo "- Building $(ARCH) Userland Applications in $(BUILD_MODE) mode"
	$(V) for APP_PRJ in $(USERLAND_APPS)/*/Cargo.toml; do                        \
              echo "- Compiling '$$(basename $$(dirname $${APP_PRJ}))'";         \
              RUSTFLAGS="$(RUSTC_FLAGS)"                                         \
              CARGO_TARGET_DIR="$(BUILD_DIR)"                                    \
                  $(CARGO) build $(CARGO_FLAGS)                                  \
                           --manifest-path $${APP_PRJ}                           \
                           --target ../$(ARCH)/userland.json; \
         done

%/Cargo.toml:
	$(V)

clean:
	$(V) echo "- Cleaning $(BUILD_MODE) build of $(ARCH)..."