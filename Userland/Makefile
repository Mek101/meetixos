#! Userland Builder Script
#!
#! This Makefile is responsible to build/install/clean/doc the Userland binaries

include ../config.mk

#
# -- -- -- -- -- -- -- -- -- -- -- Project Variables -- -- -- -- -- -- -- -- -- -- --
#

REPO_ROOT    ?= $(realpath ../)
BUILD_DIR    ?= $(REPO_ROOT)/$(BUILD_PREFIX)
BUILD_TARGET ?= $(realpath $(TARGET_PREFIX)/userland.json)

#
# -- -- -- -- -- -- -- -- -- -- -- -- Make Arguments -- -- -- -- -- -- -- -- -- -- --
#

MAKE_ARGS += REPO_ROOT="$(REPO_ROOT)" \
             BUILD_DIR="$(BUILD_DIR)" \
             BUILD_TARGET="$(BUILD_TARGET)"

#
# -- -- -- -- -- -- -- -- -- -- -- -- -- Make Targets -- -- -- -- -- -- -- -- -- -- --
#

install: build
	$(V) $(MAKE) $(MAKE_ARGS) -C Apps install
	$(V) $(MAKE) $(MAKE_ARGS) -C Bins install

build:
	$(V) $(MAKE) $(MAKE_ARGS) -C Apps build
	$(V) $(MAKE) $(MAKE_ARGS) -C Bins build

clean:
	$(V) $(RM) -rf $(BUILD_DIR)/Userland/$(BUILD_MODE)