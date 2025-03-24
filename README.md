# Micelab installer
A command line installer made for <a>Micelab</a>

>**Note: This is literally my first program written on rust, so there's a lot of improvements changes ahead. Any improvement suggestions are welcome 🗿**

### Usage (v2.0.2)
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

## Extra
Thanks to <a href="https://github.com/AstronautMarkus">AstronautMarkus</a> for testing this tool 🔥
