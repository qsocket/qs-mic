<div align="center">
  <img src=".github/img/banner.png">
  <br>
  <br>


  [![GitHub All Releases][release-img]][release]
  [![Build][workflow-img]][workflow]
  [![Issues][issues-img]][issues]
  [![Crates][crates-img]][crates]
  [![License: MIT][license-img]][license]
</div>

[crates]: https://crates.io/crates/qs-mic
[crates-img]: https://img.shields.io/crates/v/qs-mic
[release]: https://github.com/qsocket/qs-mic/releases
[release-img]: https://img.shields.io/github/v/release/qsocket/qs-mic
[downloads]: https://github.com/qsocket/qs-mic/releases
[downloads-img]: https://img.shields.io/github/downloads/qsocket/qs-mic/total?logo=github
[issues]: https://github.com/qsocket/qs-mic/issues
[issues-img]: https://img.shields.io/github/issues/qsocket/qs-mic?color=red
[license]: https://raw.githubusercontent.com/qsocket/qs-mic/master/LICENSE
[license-img]: https://img.shields.io/github/license/qsocket/qs-mic.svg
[google-cloud-shell]: https://console.cloud.google.com/cloudshell/open?git_repo=https://github.com/qsocket/qs-mic&tutorial=README.md
[workflow-img]: https://github.com/qsocket/qs-mic/actions/workflows/main.yml/badge.svg
[workflow]: https://github.com/qsocket/qs-mic/actions/workflows/main.yml
[qsrn]: https://github.com/qsocket/qsrn

qs-mic is a cross-platform microphone utility which sends microphone input across devices over the [QSRN][qsrn].

## Installation

[![Open in Cloud Shell](.github/img/cloud-shell.png)][google-cloud-shell]

|  **Tool**  |                    **Build From Source**                    |      **Docker Image**       | **Binary Release**  |
| :--------: | :---------------------------------------------------------: | :-------------------------: | :-----------------: |
| **qs-mic** | ```cargo install --git https://github.com/qsocket/qs-mic``` | [Download](#docker-install) | [Download](release) |

---
qs-mic supports 10 architectures and 12 operating systems, check **Supported Platforms** below for detailed table.

<details>
<summary>Supported Platforms</summary>

| **Platform**  | **AMD64** | **386** | **ARM** | **ARM64** | **MIPS** | **MIPS64** | **MIPS64LE** | **PPC64** | **PPC64LE** | **S390X** |
| :-----------: | :-------: | :-----: | :-----: | :-------: | :------: | :--------: | :----------: | :-------: | :---------: | :-------: |
|   **Linux**   |     ✅     |    ✅    |    ✅    |     ✅     |    ✅     |     ✅      |      ✅       |     ✅     |      ✅      |     ✅     |
|  **Darwin**   |     ✅     |    ❌    |    ❌    |     ✅     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **Windows**  |     ✅     |    ✅    |    ✅    |     ✅     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **OpenBSD**  |     ✅     |    ✅    |    ✅    |     ✅     |    ❌     |     ✅      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **NetBSD**   |     ✅     |    ✅    |    ✅    |     ✅     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **FreeBSD**  |     ✅     |    ✅    |    ✅    |     ✅     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **Android**  |     ✅     |    ✅    |    ✅    |     ✅     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|    **IOS**    |     ✅     |    ❌    |    ❌    |     ✅     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **Solaris**  |     ✅     |    ❌    |    ❌    |     ❌     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **Illumos**  |     ✅     |    ❌    |    ❌    |     ❌     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
| **Dragonfly** |     ✅     |    ❌    |    ❌    |     ❌     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|    **AIX**    |     ❌     |    ❌    |    ❌    |     ❌     |    ❌     |     ❌      |      ❌       |     ✅     |      ❌      |     ❌     |

</details>

## Usage
```
qs-mic 1.0
Ege BALCI. <egebalci@pm.me>
Qsocket microphone utility.

USAGE:
    qs-mic [OPTIONS] [--] [DEVICE]

ARGS:
    <DEVICE>    The audio device to use [default: default]

OPTIONS:
    -C, --notls                    Disable TLS encryption.
    -d, --duration [<INPUT>...]    Microphone record duration. [default: 10]
    -g, --generate                 Generate a random secret.
    -h, --help                     Print help information
    -j, --jack                     Use the JACK host
    -l, --listen                   Run in server mode. [default: client]
    -o, --out [<INPUT>...]         File name for received recording.
        --pin                      Enable certificate fingerprint verification on TLS connections.
        --play                     Play the recording while receiving.
    -q, --quiet                    Disable output.
    -s, --secret [<INPUT>...]      Secret. (e.g. password).
    -t, --probe [<INPUT>...]       Probe interval for QSRN. [default: 5]
    -v, --verbose                  Verbose output.
    -V, --version                  Print version information
```
### Examples
1. Record 10 second audio from workstation A microphone. 
```bash
$ qs-mic -l                    # Workstation A
$ qs-mic -o record.wav -d 10   # Workstation B
```
