<h1 align="center">
	replace_me 
	<a href="https://github.com/tversteeg/replace_me/releases">
		<img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/linux.svg" width="18" height="18" />
		<img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/apple.svg" width="18" height="18" />
		<img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/windows.svg" width="18" height="18" />
	</a>
</h1>
<p align="center">
	A game, you can play!
</p>
	
<p align="center">
	<a href="https://github.com/tversteeg/replace_me/actions"><img src="https://github.com/tversteeg/replace_me/workflows/CI/badge.svg" alt="CI"/></a>
	<a href="https://crates.io/crates/replace_me"><img src="https://img.shields.io/crates/v/replace_me.svg" alt="Version"/></a>
	<img src="https://img.shields.io/crates/l/replace_me.svg" alt="License"/>
	<br/>
</p>

## Play

Download the executable file from the [Releases](https://github.com/tversteeg/replace_me/releases) tab and execute it.

### Linux

You might have to change the permissions with:

```bash
chmod u+x replace_me-*
```

## Build

You will need an up-to-date [Rust](https://rustup.rs/) setup.

### Linux Dependencies

To build it on linux you will need the `xorg` & `alsa` development libraries & `cmake`:

```bash
sudo apt install xorg-dev cmake libasound2-dev
```

### Compile

You just need to run the following to compile the game after you've installed the dependencies:

```bash
cargo build --release
```

This will put the executable of the game in `target/release/`.
