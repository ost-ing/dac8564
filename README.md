# dac8564

A platform agnostic library for the Texas Instruments DAC8564.

- https://crates.io/crates/dac8564

![dac8564](/documentation/dac8564_ssop16.png)

## description

The DAC8564 is a low-power, voltage-output, four-channel, 16-bit digital-to-analog converter (DAC). The device includes a 2.5V, 2ppm/°C internal, reference (enabled by default), giving a full-scale output voltage range of 2.5V. The internal reference has an initial accuracy of 0.004% and can source up to 20mA at the VREFH/VREFOUT pin. The device is monotonic, provides very good linearity, and minimizes undesired code-to-code transient voltages (glitch). The DAC8564 uses a versatile 3-wire serial interface that operates at clock rates up to 50MHz. The interface is compatible with standard SPI™, QSPI™,  Microwire™, and digital signal processor (DSP) interfaces.

## features

- Also supports the Texas Instruments DAC7565, DAC7564, DAC8164
- Full no-std support
- Implemented with embedded-hal (https://docs.rs/embedded-hal/0.2.3/embedded_hal)
- Blocking and non-blocking support

## example

Note: Example based on the `stm32h7xx-hal`.

### blocking

```rust
fn main() -> ! {
    // SPI interface pins
    let sck = sck.into_alternate_af5();
    let mosi = mosi.into_alternate_af5();

    // Control lines
    let ldac = ldac.into_push_pull_output();
    let enable = enable.into_push_pull_output();
    let nss = nss.into_push_pull_output();

    let spi: Spi<SPI2, Enabled> = interface.spi(
        (sck, NoMiso, mosi),
        spi::MODE_0,
        20.mhz(),
        prec,
        clocks,
    );

    let mut dac = dac8564::Dac::new(spi, nss, ldac, enable);

    // Enable the DAC8564
    dac.enable();

    // Blocking call. Set value to 1000 on the DAC
    dac.write(Channel::A, 1000).unwrap();
}
```

## contributing

I am not actively using the `DAC8564` chip in any of my current prototypes. For this reason development, features and bug fixes could be slow, but I am fully open to any contribution. Please create a PR if you have any changes 🙏🏼
