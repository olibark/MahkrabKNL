SHELL := /bin/sh

TARGET := x86_64-unknown-uefi
PROFILE ?= debug
QEMU ?= qemu-system-x86_64

CARGO := cargo
CARGO_TARGET_DIR ?= target
BUILD_DIR := $(CARGO_TARGET_DIR)/$(TARGET)/$(PROFILE)
EFI_BINARY := $(BUILD_DIR)/bootloader.efi
ESP_DIR := esp
ESP_BOOT_DIR := $(ESP_DIR)/EFI/BOOT
ESP_BOOT_EFI := $(ESP_BOOT_DIR)/BOOTX64.EFI
KERNEL_ELF ?= kernel.elf
ESP_KERNEL_ELF := $(ESP_BOOT_DIR)/kernel.elf
TEST_KERNEL_DIR := test_kernel
TEST_KERNEL_ASM := $(TEST_KERNEL_DIR)/entry.S
TEST_KERNEL_LINKER := $(TEST_KERNEL_DIR)/linker.ld
TEST_KERNEL_OBJ := target/test-kernel/entry.o
OVMF_VARS_LOCAL := OVMF_VARS.fd

OVMF_CODE := $(firstword $(wildcard \
	/usr/share/OVMF/OVMF_CODE_4M.fd \
	/usr/share/qemu/OVMF.fd \
	/usr/share/ovmf/OVMF.fd))

OVMF_VARS_TEMPLATE := $(firstword $(wildcard \
	/usr/share/OVMF/OVMF_VARS_4M.fd \
	/usr/share/ovmf/OVMF_VARS.fd))

.PHONY: all help build test-kernel esp run clean FORCE

all: build

help:
	@printf '%s\n' \
		'make build  - build the UEFI application' \
		'make test-kernel - build and stage an ELF loader exercise kernel.elf' \
		'make esp    - stage BOOTX64.EFI and kernel.elf into ./esp/EFI/BOOT' \
		'make run    - boot the app in QEMU with OVMF' \
		'make clean  - remove build and local run artifacts' \
		'' \
		'Set KERNEL_ELF=/path/to/kernel.elf if the kernel is not ./kernel.elf'

build: $(EFI_BINARY)

$(EFI_BINARY): FORCE
	CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) $(CARGO) build --target $(TARGET) $(if $(filter release,$(PROFILE)),--release,)

$(TEST_KERNEL_OBJ): $(TEST_KERNEL_ASM)
	mkdir -p $(dir $@)
	as --64 -o $@ $<

kernel.elf: $(TEST_KERNEL_OBJ) $(TEST_KERNEL_LINKER)
	ld -nostdlib -m elf_x86_64 -T $(TEST_KERNEL_LINKER) -o $@ $(TEST_KERNEL_OBJ)

test-kernel: kernel.elf | $(ESP_BOOT_DIR)
	cp kernel.elf $(ESP_KERNEL_ELF)

$(ESP_BOOT_DIR):
	mkdir -p $(ESP_BOOT_DIR)

$(ESP_BOOT_EFI): $(EFI_BINARY) | $(ESP_BOOT_DIR)
	cp $(EFI_BINARY) $(ESP_BOOT_EFI)

$(ESP_KERNEL_ELF): FORCE | $(ESP_BOOT_DIR)
	@if [ -f "$(KERNEL_ELF)" ]; then \
		cp "$(KERNEL_ELF)" "$(ESP_KERNEL_ELF)"; \
	elif [ -f "$(ESP_KERNEL_ELF)" ]; then \
		echo "Using existing $(ESP_KERNEL_ELF)"; \
	else \
		echo "Missing kernel ELF: $(KERNEL_ELF)"; \
		echo "Build or copy your kernel there, run: make test-kernel, or run: make esp KERNEL_ELF=/path/to/kernel.elf"; \
		exit 1; \
	fi

FORCE:

esp: $(ESP_BOOT_EFI) $(ESP_KERNEL_ELF)

$(OVMF_VARS_LOCAL):
	@test -n "$(OVMF_CODE)" || { echo "Missing OVMF firmware. Install ovmf."; exit 1; }
	@test -n "$(OVMF_VARS_TEMPLATE)" || { echo "Missing OVMF vars template. Install ovmf."; exit 1; }
	cp $(OVMF_VARS_TEMPLATE) $(OVMF_VARS_LOCAL)

run: $(OVMF_VARS_LOCAL) esp
	$(QEMU) \
		-machine q35,accel=tcg \
		-m 256M \
		-drive if=pflash,format=raw,readonly=on,file=$(OVMF_CODE) \
		-drive if=pflash,format=raw,file=$(OVMF_VARS_LOCAL) \
		-drive format=raw,file=fat:rw:$(ESP_DIR)

clean:
	$(CARGO) clean
	rm -rf $(ESP_DIR) $(OVMF_VARS_LOCAL)
