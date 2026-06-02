SHELL := /bin/sh

TARGET := x86_64-unknown-uefi
PROFILE ?= debug
QEMU ?= qemu-system-x86_64

CARGO := cargo
BUILD_DIR := target/$(TARGET)/$(PROFILE)
EFI_BINARY := $(BUILD_DIR)/bootloader.efi
ESP_DIR := esp
ESP_BOOT_DIR := $(ESP_DIR)/EFI/BOOT
ESP_BOOT_EFI := $(ESP_BOOT_DIR)/BOOTX64.EFI
OVMF_VARS_LOCAL := OVMF_VARS.fd

OVMF_CODE := $(firstword $(wildcard \
	/usr/share/OVMF/OVMF_CODE_4M.fd \
	/usr/share/qemu/OVMF.fd \
	/usr/share/ovmf/OVMF.fd))

OVMF_VARS_TEMPLATE := $(firstword $(wildcard \
	/usr/share/OVMF/OVMF_VARS_4M.fd \
	/usr/share/ovmf/OVMF_VARS.fd))

.PHONY: all help build esp run clean

all: build

help:
	@printf '%s\n' \
		'make build  - build the UEFI application' \
		'make esp    - stage BOOTX64.EFI into ./esp' \
		'make run    - boot the app in QEMU with OVMF' \
		'make clean  - remove build and local run artifacts'

build:
	$(CARGO) build --target $(TARGET) $(if $(filter release,$(PROFILE)),--release,)

$(ESP_BOOT_DIR):
	mkdir -p $(ESP_BOOT_DIR)

$(ESP_BOOT_EFI): $(EFI_BINARY) | $(ESP_BOOT_DIR)
	cp $(EFI_BINARY) $(ESP_BOOT_EFI)

esp: $(ESP_BOOT_EFI)

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
