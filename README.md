<div align='center'>

# kitget

Display and customize cat images in your terminal :smirk_cat:

![kitget](./res/kitget.gif)

</div>

## Usage

|                                   |                                   |
|:----------------------------------|:----------------------------------|
| ![preview 0](./res/preview0.webp) | ![preview 1](./res/preview1.webp) |
| ![preview 2](./res/preview2.webp) | ![preview 3](./res/preview3.webp) |

### Fastfetch integration

You can use kitget directly with Fastfetch:

```bash
kitget --square | fastfetch --file-raw -
```

This might not work on terminals supporting images. Example Bash function (to use in your `~/.bashrc`) for Kitty:

```bash
ff() {
    stamp="$(date +%s)"
    kitget --square -o "/tmp/kitget-$stamp"
    clear
    fastfetch --kitty "/tmp/kitget-$stamp" "$@"
    rm -f "/tmp/kitget-$stamp"
}
```

## Installation

<details>
<summary>Arch Linux</summary>

[kitget](https://aur.archlinux.org/packages/kitget) is available in the AUR.
It can be installed using an AUR helper (e.g. paru):

```bash
paru -S kitget
```

</details>
<details>
<summary>Cargo</summary>

For all systems supported by Rust, you can install kitget using Cargo:

```bash
cargo install kitget
```

Keep in mind that crates installed with `cargo install` have to be manually upgraded and may not be included in `$PATH` by default.

</details>

## Thanks to

- [Kevin Balicot](https://buymeacoffee.com/kevinbalicot) for the [Cat as a service API][cataas] (go give him money)
- [Sebaguardian](https://github.com/Sebaguardian) for mental support

[cataas]: https://cataas.com
