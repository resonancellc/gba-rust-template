Emulator = mgba

all:
	xargo build --target gba --release
	arm-none-eabi-as -o target/gba/release/begin.o src/begin.s
	arm-none-eabi-ld -T linker.ld -o  target/gba/release/game.elf target/gba/release/begin.o target/gba/release/libgame.a
	arm-none-eabi-objcopy -O binary target/gba/release/game.elf target/gba/release/game.gba
	gbafix target/gba/release/game.gba

rust:
	xargo build --target gba --release

run: all
	$(Emulator) target/gba/release/game.gba
