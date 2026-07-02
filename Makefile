SHELL := /bin/sh

UEFI_TARGET := x86_64-unknown-uefi
PROFILE ?= debug
QEMU ?= qemu-system-x86_64

CARGO := cargo
TARGET_DIR ?= $(CURDIR)/target
DIST_DIR ?= $(CURDIR)/dist
ESP_DIR := $(DIST_DIR)/esp
ESP_BOOT_DIR := $(ESP_DIR)/EFI/BOOT
ESP_BOOT_EFI := $(ESP_BOOT_DIR)/BOOTX64.EFI
ESP_KERNEL_ELF := $(ESP_BOOT_DIR)/kernel.elf

BOOTLOADER_EFI := $(TARGET_DIR)/$(UEFI_TARGET)/$(PROFILE)/bootloader.efi
KERNEL_DIR := $(CURDIR)/kernel
KERNEL_BUILD_DIR := $(TARGET_DIR)/kernel
KERNEL_ELF := $(KERNEL_BUILD_DIR)/kernel.elf

OVMF_VARS := $(TARGET_DIR)/OVMF_VARS.fd
PROOF_DIR := $(TARGET_DIR)/proof
PROOF_LOG ?= $(PROOF_DIR)/kernel-entry-proof.log
PROOF_QEMU_LOG ?= $(PROOF_DIR)/kernel-entry-qemu.log
PROOF_OVMF_VARS := $(PROOF_DIR)/OVMF_VARS.fd
PROOF_TIMEOUT ?= 30s

PROFILE_FLAG := $(if $(filter release,$(PROFILE)),--release,)

OVMF_CODE := $(firstword $(wildcard \
	/usr/share/OVMF/OVMF_CODE_4M.fd \
	/usr/share/qemu/OVMF.fd \
	/usr/share/ovmf/OVMF.fd))

OVMF_VARS_TEMPLATE := $(firstword $(wildcard \
	/usr/share/OVMF/OVMF_VARS_4M.fd \
	/usr/share/ovmf/OVMF_VARS.fd))

.PHONY: all help build build-bootloader build-kernel stage run prove-entry clean

all:
	$(MAKE) clean
	$(MAKE) run

help:
	@printf '%s\n' \
		'make all         - clean, rebuild, stage, and run in QEMU' \
		'make build       - build bootloader and kernel artifacts' \
		'make stage       - stage BOOTX64.EFI and kernel.elf into dist/esp' \
		'make run         - stage artifacts and boot QEMU with OVMF' \
		'make prove-entry - run headless QEMU and verify debugcon proof markers' \
		'make clean       - remove build, staging, and local run artifacts' \
		'' \
		'Options: PROFILE=debug|release QEMU=qemu-system-x86_64 PROOF_TIMEOUT=30s'

build: build-bootloader build-kernel

build-bootloader:
	CARGO_TARGET_DIR=$(TARGET_DIR) $(CARGO) build -p bootloader --target $(UEFI_TARGET) $(PROFILE_FLAG)

build-kernel:
	$(MAKE) -C $(KERNEL_DIR) ROOT_DIR=$(CURDIR) TARGET_DIR=$(TARGET_DIR) KERNEL_ELF=$(KERNEL_ELF) build

stage: build | $(ESP_BOOT_DIR)
	cp $(BOOTLOADER_EFI) $(ESP_BOOT_EFI)
	cp $(KERNEL_ELF) $(ESP_KERNEL_ELF)

$(ESP_BOOT_DIR):
	mkdir -p $@

$(TARGET_DIR):
	mkdir -p $@

$(OVMF_VARS): | $(TARGET_DIR)
	@test -n "$(OVMF_CODE)" || { echo "Missing OVMF firmware. Install ovmf."; exit 1; }
	@test -n "$(OVMF_VARS_TEMPLATE)" || { echo "Missing OVMF vars template. Install ovmf."; exit 1; }
	cp $(OVMF_VARS_TEMPLATE) $@

run: stage $(OVMF_VARS)
	$(QEMU) \
		-machine q35,accel=tcg \
		-m 256M \
		-drive if=pflash,format=raw,readonly=on,file=$(OVMF_CODE) \
		-drive if=pflash,format=raw,file=$(OVMF_VARS) \
		-drive format=raw,file=fat:rw:$(ESP_DIR)

prove-entry: stage
	@test -n "$(OVMF_CODE)" || { echo "Missing OVMF firmware. Install ovmf."; exit 1; }
	@test -n "$(OVMF_VARS_TEMPLATE)" || { echo "Missing OVMF vars template. Install ovmf."; exit 1; }
	@mkdir -p $(PROOF_DIR)
	@rm -f $(PROOF_LOG) $(PROOF_QEMU_LOG)
	@cp $(OVMF_VARS_TEMPLATE) $(PROOF_OVMF_VARS)
	@status=0; \
	timeout $(PROOF_TIMEOUT) $(QEMU) \
		-machine q35,accel=tcg \
		-m 256M \
		-display none \
		-monitor none \
		-serial file:$(PROOF_QEMU_LOG) \
		-debugcon file:$(PROOF_LOG) \
		-global isa-debugcon.iobase=0xe9 \
		-drive if=pflash,format=raw,readonly=on,file=$(OVMF_CODE) \
		-drive if=pflash,format=raw,file=$(PROOF_OVMF_VARS) \
		-drive format=raw,file=fat:rw:$(ESP_DIR) >>$(PROOF_QEMU_LOG) 2>&1 || status=$$?; \
	cat $(PROOF_QEMU_LOG); \
	cat $(PROOF_LOG); \
	if [ $$status -ne 0 ] && [ $$status -ne 124 ]; then exit $$status; fi; \
	grep -q 'KERNEL ENTRY REACHED' $(PROOF_LOG) || { \
		echo 'Missing kernel entry proof marker in $(PROOF_LOG)'; \
		wc -c $(PROOF_LOG); \
		wc -c $(PROOF_QEMU_LOG); \
		exit 1; \
	}; \
	grep -q 'FRAMEBUFFER PAINTED' $(PROOF_LOG) || { \
		echo 'Missing framebuffer proof marker in $(PROOF_LOG)'; \
		wc -c $(PROOF_LOG); \
		wc -c $(PROOF_QEMU_LOG); \
		exit 1; \
	}

clean:
	rm -rf $(TARGET_DIR) $(DIST_DIR) bootloader/target bootloader/esp
	rm -f bootloader/OVMF_VARS.fd bootloader/kernel.elf
