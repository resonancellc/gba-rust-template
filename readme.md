# GBA Rust Template
this is a small repository, you can clone for starting a new Rust project for the GBA!

It uses the [library](https://github.com/jkarns275/stdgba).

## Dependencies
sadly there is quite a lot you need to have installed.

- Rust :^)
- Xargo
- Make
- arm-none-eabi-binutils
- gba-tools

I hope I haven't missed one...

now some help on how to install all of them:

#### Xargo
Xargo can be installed via cargo, like this:
```
cargo install xargo
```
don't forget to add "~/.cargo/bin/" to your Path!

#### Make
Make should already be installed on your system, if it isn't then your OS should provide a package called "make", probably.

#### arm-none-eabi-binutils
I have almost no clue what this really is to be honest. all I know is that its a part of the GNU Build System but compiles for ARM instead of your system.

You should be able to install it via your package manager, at least Debian Ubuntu and Arch do have a package called "arm-none-eabi-binutils".

#### gba-tools
I compiled this myself and you probably also have to do that.

You can get the code from [here](https://github.com/devkitPro/gba-tools).

and here's how to do it: (mainly because I'll forget how to in like a week)
```
git clone https://github.com/devkitPro/gba-tools
cd gba-tools
./autogen.sh
./configure
make
make install
```

now you should have the command "gbafix".

## Usage
#### Getting started
just clone this repository:
```
git clone https://github.com/AlexFence/gba-rust-template
```

and run the init script I wrote:
```
cd gba-rust-template
./init.sh
```

You will then be asked for a project name for the Cargo.toml.

The script will set you as the author in the Cargo.toml, it takes that info from your git configuration.

If you wish to edit the Cargo.toml manually, just run the "remove-init.sh" script.
```
./remove-init.sh
```

#### How to compile/run your project
This template is setup in a way that it uses make for most things.
I know its somewhat unusual for Rust but it makes some stuff easier, sadly the Rust internal Build scripts won't let you run stuff post compilation.

The makefile is fairly simple, there are the following targets:

- rust
- all
- doc
- run
- clean

you can run those with:
```
# replace target with the name of the target you want to run
make target
```

##### rust
This only compiles the rust code. You could also run:
```
xargo build --release
```

##### all
This builds your rust code and some assembly and links it and I dunno...

Basically it does some magic and you'll get a nice rom you can run with an emulator, called "game.gba" in "./target/gba/release/".

##### doc
This builds the documentation of your project. You could also run:
```
xargo doc --open
```

##### run
This runs the target "all" and then opens the rom with an emulator.

By default it runs "mgba". You can change the command, by changing the Emulator variable in the makefile.

##### clean
This deletes all the binaries, located at "./target". You could also run:
```
xargo clean
```

## Credits
The Repositry of the Library doesn't have a license as of now, **BUT** I was told by the author that I could do what ever with his gba related projects.
He also told me that he would add one, but hasn't done it as of now.

I pulled the files:
- /src/begin.s
- linker.ld
- gba.json

and some parts of */src/lib.rs* from [here](https://github.com/jkarns275/g).
