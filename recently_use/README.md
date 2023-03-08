# recently_use

Adds a file to gtk's "recently used" list.
## Bugs
* Currently, a recently-used.xbel file needs to exist before this will add files to it

# Building
```sh
cargo build
```
or
```sh
nix build
```

# Running
```
cargo run -- /path/to/use/here
```
or
```
nix run /path/to/use/here
```

# Installing
This program is available as a Nix Flake (TODO: flake install instructions)

I don't usually do this, but you can probably
```sh
cargo install
```
