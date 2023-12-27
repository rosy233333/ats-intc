# ats-intc
This is the driver of ats-intc(Asynchronous task scheduler and Interrupt controller)

### PAC

This IO registers are specificed by `svd`, if you want to modify the specification, you must install the required toolchain with follow commands:

```bash
cargo install svd2rust
cargo install form
```

Then you should create a new Peripheral access crate, for example:

```bash
cargo new xxx-pac --lib
```

The `cargo.toml` must has the follow dependencies:

```rust
[dependencies]
critical-section = { version = "1.0", optional = true }
cortex-m = "0.7.6"
cortex-m-rt = { version = "0.7.3", optional = true }
vcell = "0.1.2"

[features]
rt = ["cortex-m-rt/device"]
```

After you have done the above operations, you can modify the svd file (The svd file should not under the `src` directory). After finishing modifying the svd file, you can use the `genpac.py` script to generate the hareware pac crate. 

```bash
python3 $(target_svd_filename)
```
