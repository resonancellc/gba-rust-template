Emulator = mgba

rust:
	xargo build --release

all: rust
	arm-none-eabi-as -o target/gba/release/begin.o src/begin.s
	arm-none-eabi-ld -T linker.ld -o  target/gba/release/game.elf target/gba/release/begin.o target/gba/release/libgame.a
	arm-none-eabi-objcopy -O binary target/gba/release/game.elf target/gba/release/game.gba
	gbafix target/gba/release/game.gba

doc:
	xargo doc --open

run: all
	$(Emulator) target/gba/release/game.gba

clean:
	xargo clean
