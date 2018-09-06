# `max32630`

> Peripheral access API for Maxim Integrated 32630 devices

## `max32630_svd`

# [Documentation](https://docs.rs/max32630_svd)

The SVD file was extracted from the pack provided by Maxim available
from here: <https://www.keil.com/dd2/pack/>

We use `svd2rust` to generate the code, `form` to organize it, and `cargo fmt` to make it pretty

Issues:

* The example startup code on mbed references a number of registers that are not present in the IOMAN portion of the SVD file.  [ioman_regs.h](https://github.com/ARMmbed/mbed-os/blob/master/targets/TARGET_Maxim/TARGET_MAX32630/device/ioman_regs.h#L616).  The User Guide linked below has technical info that will help to fill in the SVD file appropriately.

## Other Resources

* [mbed-os target](https://github.com/ARMmbed/mbed-os/tree/master/targets/TARGET_Maxim/TARGET_MAX32630)
* [startup code](https://github.com/ARMmbed/mbed-os/blob/master/targets/TARGET_Maxim/TARGET_MAX32630/TARGET_MAX32630FTHR/low_level_init.c)
* [Product Description](https://www.maximintegrated.com/en/products/microcontrollers/MAX32630FTHR.html)
* [MAX32630 Rev B User Guide](https://pdfserv.maximintegrated.com/en/an/AN6349.pdf)
* [MAX32630 Datasheet](https://datasheets.maximintegrated.com/en/ds/MAX32630-MAX32631.pdf)
* [MAX32630FTHR Datasheet](https://datasheets.maximintegrated.com/en/ds/MAX32630FTHR.pdf)

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
