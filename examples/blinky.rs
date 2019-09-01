#![no_std]

extern crate msp430;
extern crate msp430fr2355;
extern crate panic_msp430;

use msp430::{asm, interrupt};
use msp430fr2355::Peripherals;

fn delay(n: u16) {
    let mut i = 0;
    loop {
        asm::nop();

        i += 1;

        if i == n {
            break;
        }
    }
}

// P0 = red LED
// P6 = green LED
fn main() {
    let peripherals = Peripherals::take().unwrap();

    interrupt::free(|_| {
        // Disable watchdog
        let wdt = peripherals.WDT_A;
        wdt.wdtctl
            .write(|w| unsafe { w.wdtpw().bits(0x5A) }.wdthold().hold());

        peripherals.PMM.pm5ctl0.write(|w| w.locklpm5().locklpm5_0());

        let port1 = peripherals.P1;
        port1.p1dir.write(|w| unsafe { w.bits(0xFF) });

        let port6 = peripherals.P6;
        port6.p6dir.write(|w| unsafe { w.bits(0xFF) });

        loop {
            delay(10_000);

            // toggle outputs
            port1.p1out.modify(|r, w| unsafe { w.bits(!r.bits()) });
            port6.p6out.modify(|r, w| unsafe { w.bits(!r.bits()) });
        }
    });
}
