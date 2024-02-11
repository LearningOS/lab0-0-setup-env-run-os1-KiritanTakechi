# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
BIOS := bootloader/rustsbi-qemu.bin
KERNEL_ELF := target/$(TARGET)/$(MODE)/libos
KERNEL_BIN := $(KERNEL_ELF).bin

# KERNEL ENTRY
KERNEL_ENTRY_PA := 0x80200000

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# QEMU
QEMU_ARGS := -machine virt -nographic -bios $(BIOS) -device loader,file=$(KERNEL_ELF),addr=$(KERNEL_ENTRY_PA)

build:
	@cargo build --$(MODE) --target $(TARGET)

run: build
	@qemu-system-riscv64 $(QEMU_ARGS)

gdbserver: build
	@qemu-system-riscv64 $(QEMU_ARGS) -s -S

gdbclient:
	@lldb --file $(KERNEL_ELF) -o 'gdb-remote localhost:1234'

clean:
	@cargo clean

.PHONY: build run clean