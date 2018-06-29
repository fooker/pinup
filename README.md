# Pinup - Triggers scripts on GPIO events
Pinup is made to execute scripts if someone pushes a button.
The daemon listens for events on GPIO pins and connected to a push button.
It uses the configuration to define which button and therefore which script is attached to which GPIO pin.
Additionally it provides settings to configure different kinds of button characteristics.

## Install
To install the latest released version of pinup, ensure that the latest stable version of Rust has been installed and run:

```
cargo build -t release
```

## Configuration
The configuration is defined in a YAML file.
See `contrib/config.example.yaml` for an example.
