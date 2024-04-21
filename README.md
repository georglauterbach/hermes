# _Hermes_

## :mailbox: About

_Hermes_ configures _Ubuntu_ by installing various packages and placing (new) configuration files. The configuration enhances the out-of-the-box experience of Ubuntu. It will automatically detect the installed version (starting from _Ubuntu_ 23.10 "_Mantic Minotaur_"). You can also optionally install and configure the GUI.

> [!NOTE]
>
> We do not support platforms apart from `amd64` because differentiating platforms involves a complexity explosion for this project.

## :rocket: Usage

The installation script can be downloaded and executed in the terminal.

```console
$ wget https://raw.githubusercontent.com/georglauterbach/hermes/main/setup.sh
$ bash setup.sh
...
```

If you want to configure your graphical user interface too, you can use the `--gui` flag. With this flag, additional programs are installed that are only necessary when you have a GUI.

You may also use the `--local` flag when you cloned this repository. This flag will causes the setup to be completely local, without requiring an internet connection. Only this repository has to be cloned in its entirety.

## :mega: Supplementary Projects

You might want to take a look at the following outstanding projects. Their installation does not yet come with _Hermes_.

### General

1. [`akinomyoga/ble.sh`](https://github.com/akinomyoga/ble.sh): command line editor that replaces the default GNU Readline
2. [`volian/nala`](https://gitlab.com/volian/nala): frontend for `libapt-pkg`

### Written in Rust

> [!TIP]
>
> Check out [`cargo-bins/cargo-binstall`](https://github.com/cargo-bins/cargo-binstall) first. This way, you may be able to save yourself time by not requiring local compilation; use `cargo binstall` instead of `cargo install`.

1. [`mozilla/sccache`](https://github.com/mozilla/sccache): compiler wrapper that avoids compilation when possible
2. [`Canop/bacon`](https://github.com/Canop/bacon): background Rust code checker
3. [`zellij-org/zellij`](https://github.com/zellij-org/zellij): terminal workspace (multiplexer)
4. [`casey/just`](https://github.com/casey/just): command runner
