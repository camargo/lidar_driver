# lidar_driver

This is an example driver in Rust for an emulated LIDAR (Light Detection and Ranging) sensor.

## Crates

| Name                   | Description                                                    |
| ---------------------- | -------------------------------------------------------------- |
| [driver](./driver)     | Lidar driver for commanding and receiving data from the sensor |
| [emulator](./emulator) | Lidar emulator since we are not working on hardware            |

## Install Dependencies

This project assumes you are on Ubuntu `22.04`. First install [Rust 1.80.1](https://www.rust-lang.org/learn/get-started), and also install the following Linux dependencies:

```sh
apt install -y build-essential git libudev-dev pkg-config socat
```

There is also a [.devcontainer](./.devcontainer) you can use or reference for a more reproducible dev environment (requires [Docker](https://www.docker.com/) and [VS Code](https://code.visualstudio.com/) to use) (recommended!).

## Build and Run

You can use the [run.sh](./run.sh) script to build and run everything in a single terminal:

```sh
./run.sh
```

## Send Commands

The Lidar only has two commands, which switch it between two states: Standby and Scan. In a new split terminal (with the `run.sh` terminal still in view), you can switch to Scan via:

```sh
echo -n -e '\x01\x00\x00\x00' > /tmp/command_out
```

You can switch to Standby via:

```sh
echo -n -e '\x00\x00\x00\x00' > /tmp/command_out
```

Notice the driver prints the current state information so you can observe the sensor state changing in real-time.
