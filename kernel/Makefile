SHELL := /bin/sh

ROOT_DIR ?= ..
TARGET_DIR ?= $(ROOT_DIR)/target
BUILD_DIR := $(TARGET_DIR)/kernel
KERNEL_ELF ?= $(BUILD_DIR)/kernel.elf
KERNEL_OBJ := $(BUILD_DIR)/entry.o

AS ?= as
LD ?= ld

.PHONY: all build clean print-artifact

all: build

build: $(KERNEL_ELF)

$(KERNEL_ELF): $(KERNEL_OBJ) linker.ld
	$(LD) -nostdlib -m elf_x86_64 -T linker.ld -o $@ $(KERNEL_OBJ)

$(KERNEL_OBJ): entry.S | $(BUILD_DIR)
	$(AS) --64 -o $@ $<

$(BUILD_DIR):
	mkdir -p $@

print-artifact: build
	@printf '%s\n' '$(KERNEL_ELF)'

clean:
	rm -rf $(BUILD_DIR)
