# Micelab installer
A command line installer made for <a>Micelab</a>

>**Note: This is literally my first program written on rust, so there's a lot of improvements changes ahead. Any improvement suggestions are welcome 🗿**

### Usage (v2.1.2)
```bash
mlab [command]
```
### Commands
```
  help => Show help
  install => Install MiceLab
  env:debug => Change the app to debug mode
  env:production => Change the app to production mode
  env:check => Check if app is on production or debug,
  uninstall => Uninstall MiceLab 
  version  => Shows cli version
```

# Build
**Pre-requisites:**
it's been assumed that you have:
- rust
- cargo

```bash
cargo build --release # Production use version. Without tests commands.
cargo build # Debug, development version. With test commands
```
  
### Dependencies
In order to build by your own, you must need some dependencies installed on your system:
### Linux
```bash
    # For debian
    sudo apt install gcc libssl-dev libsqlite3-dev
```
```bash
    # For RHEL/Fedora
    sudo dnf install gcc openssl-devel sqlite-devel
```
> **Note:** Write an issue if i missed one
