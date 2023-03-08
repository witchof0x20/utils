# Yaru

`yaru` (やる - casual Japanese for "to do") is a tool to do things with a profile.

`ssh` is nice. you can `ssh mymachine`, and it will load in a bunch of stuff from `~/.ssh/config` then just log in. `mymachine` is an alias for a bunch of settings!

Originally I wanted something similar with XFreeRDP because I used to have to rdp into a bunch of different machines and ssh-like profiles would be nice (TODO: implement this). Right now it only supports

* git profiles
* Linux bluetooth stuff

I could make this dynamic, and have it load command templates from a config file, but the 2 things I started to implement it for have nice rust bindings (nicer than shelling out to `std::process::Command`) so I decided to just use those instead. 

This basically just supports doing stuff on GNU/Linux that I wish I had profiles for. If for whatever reason you find this and want to add something, I'll start looking into cargo feature flags and making a `CONTRIBUTING.md`.

# How to use
NixOS
```sh
# Building
nix build .#
# Running
nix run .# -- help 
``` 
Otherwise
```sh
# Building
cargo build
# or
cargo build --release

# Running
cargo run -- argsgohere
# or
cargo run --release -- argsgohere
```
