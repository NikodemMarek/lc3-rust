# lc3-rust

This is [Little Computer 3](https://en.wikipedia.org/wiki/Little_Computer_3) emulator, written in Rust  
It supports all instructions (except RTI which is unused in a vm) and high-level implementation of trap routines


## Run Locally

Clone the project

```bash
git clone https://github.com/NikodemMarek/lc3-rust.git
```

Go to the project directory

```bash
cd lc3-rust
```

Run the emulator

```bash
cargo run -- <filename>
```

where `<filename>` is a name of the file to run (`hello-world.obj`, `2048.obj`, `rogue.obj`)


## Running Tests

To run tests, run the following command

```bash
cargo test
```


## Usage/Examples

2048

```bash
cargo run -- 2048.obj

Control the game using WASD keys.
Are you on an ANSI terminal (y/n)? y

+--------------------------+
|                          |
|         2                |
|                          |
|                          |
|                          |
|   2                      |
|                          |
|                          |
|                          |
+--------------------------+
```

rogue

```bash
cargo run -- rogue.obj

Welcome to LC3 Rogue.
Use WSAD to move.
Press any key..

##################  ############
###################     ########
#######################        #
########################  #  #
###############################D
################################
################################
@ ##############################
#  #############################
##    ##########################
#####  #########################
######  ########################
#######   ######################
#########    ###################
############  ##  ##############
#############      #############
```
