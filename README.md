# 🪢 TFR - Template file renamer

A tool for fast renaming or moving files according to a given input and output templates. An analog to linux mmv

Project on the HSE Rust course

## 🔧 Installation

You must have the project [dependencies](#-dependencies) installed

1. Cloning a repository

```shell
git clone https://github.com/X-OrBit/tfr
```

2. Going to the tfr directory

```shell
cd tfr
```

3. Building

```shell
cargo build -p tfr -r
```

The binary file will be located along the path `./target/release/tfr`

## 📦 Releases

Releases and builds of the program can be found at the [link](https://github.com/X-OrBit/tfr/releases)

## 👔 Dependencies

For this project you must have installed Rust compiler and cargo:

### MacOS

#### Homebrew
```shell
sudo brew install rust
```

#### MacPorts
```shell
sudo port install rust
```

### MacOS, Linux and other Unix-like OS

Run the following in terminal, then follow the on-screen instructions:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows

Download and run [rustup-init.exe](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)


## 🚀 Usage

Moving single file

```shell
tfr source/file/path destination/file/path
```

Moving all files from directory

```shell
tfr source/dir/path/* destination/file/path/#1
```

Moving only `.txt` files from directory
```shell
tfr source/dir/path/*.txt destination/file/path/#1.txt
```

Change file name formats
```shell
tfr source/dir/path/image_*_from_*.* destination/file/path/#2_#1_image.#3
```

## ⚠️ Possible problems

There may be problems on systems where the file system does not support `/`


## ☑️ TODO list
- [ ] Support for capture flags in directories (`source/dir_*/path/*.png`)
- [ ] Support for  including `/` in captures with special capture flag: `**` (`source/**/path/**.png`)
- [ ] Support for moving/renaming directories (`source/directory/path/to/move`)
- [ ] Support for insertion flags in directories (`destination/dir_#1/path/#2.png`)