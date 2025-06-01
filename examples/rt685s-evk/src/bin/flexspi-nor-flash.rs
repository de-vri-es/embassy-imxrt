#![no_std]
#![no_main]

use embassy_imxrt::flexspi::nor_flash::{FlashAlignment, FlexSpiNorFlash};
use defmt_rtt as _;
use mimxrt600_fcb::FlexSPIFlashConfigurationBlock;

// auto-generated version information from Cargo.toml
include!(concat!(env!("OUT_DIR"), "/biv.rs"));

#[link_section = ".otfad"]
#[used]
static OTFAD: [u8; 256] = [0; 256];

#[rustfmt::skip]
#[unsafe(link_section = ".fcb")]
#[used]
static FCB: FlexSPIFlashConfigurationBlock = const { FlexSPIFlashConfigurationBlock::build().lookup_table(flexspi_lut()) };

const fn flexspi_lut() -> [u32; 64] {
    use mimxrt600_fcb::flexspi_lut_seq;
    use mimxrt600_fcb::FlexSpiLutOpcode::{CMD_DDR, CMD_SDR, DUMMY_DDR, RADDR_DDR, READ_DDR, READ_SDR, STOP, WRITE_DDR, WRITE_SDR};
    use mimxrt600_fcb::FlexSpiNumPads::{Single, Octal};
    [
        // 0: Read
        flexspi_lut_seq(CMD_DDR, Octal, 0xee, CMD_DDR, Octal, 0x11),
        flexspi_lut_seq(RADDR_DDR, Octal, 32, DUMMY_DDR, Octal, 41),
        flexspi_lut_seq(READ_DDR, Octal, 0x04, STOP, Single, 0x00),
        0,
        // 1: Read status SPI
        flexspi_lut_seq(CMD_SDR, Single, 0x05, READ_SDR, Single, 0x04),
        0,
        0,
        0,
        // 2: Read status OPI
        flexspi_lut_seq(CMD_DDR, Octal, 0x05, CMD_DDR, Octal, 0xFA),
        flexspi_lut_seq(RADDR_DDR, Octal, 32, DUMMY_DDR, Octal, 20),
        flexspi_lut_seq(READ_DDR, Octal, 0x04, STOP, Single, 0x00),
        0,
        // 3: Write enable
        flexspi_lut_seq(CMD_SDR, Single, 0x06, STOP, Single, 0x00),
        0,
        0,
        0,
        // 4: Write enable - OPI
        flexspi_lut_seq(CMD_DDR, Octal, 0x06, CMD_DDR, Octal, 0xF9),
        0,
        0,
        0,
        // 5: Erase Sector
        flexspi_lut_seq(CMD_DDR, Octal, 0x21, CMD_DDR, Octal, 0xDE),
        flexspi_lut_seq(RADDR_DDR, Octal, 32, STOP, Single, 0x00),
        0,
        0,
        // 6: Enable OPI DDR mode (custom, refered to in FCB)
        flexspi_lut_seq(CMD_SDR, Single, 0x72, CMD_SDR, Single, 0x00),
        flexspi_lut_seq(CMD_SDR, Single, 0x00, CMD_SDR, Single, 0x00),
        flexspi_lut_seq(CMD_SDR, Single, 0x00, WRITE_SDR, Single, 0x01),
        0,
        // 7: Read Identification (custom, for manual use)
        flexspi_lut_seq(CMD_DDR, Octal, 0x9F, CMD_DDR, Octal, 0x60),
        flexspi_lut_seq(CMD_DDR, Octal, 0x00, CMD_DDR, Octal, 0x00),
        flexspi_lut_seq(CMD_DDR, Octal, 0x00, CMD_DDR, Octal, 0x00),
        flexspi_lut_seq(DUMMY_DDR, Octal, 9, READ_DDR, Octal, 4),
        // 8: Erase block
        flexspi_lut_seq(CMD_DDR, Octal, 0xDC, CMD_DDR, Octal, 0x23),
        flexspi_lut_seq(RADDR_DDR, Octal, 0x20, STOP, Single, 0x00),
        0,
        0,
        // 9: Page program
        flexspi_lut_seq(CMD_DDR, Octal, 0x12, CMD_DDR, Octal, 0xED),
        flexspi_lut_seq(RADDR_DDR, Octal, 0x20, WRITE_DDR, Octal, 0x04),
        0,
        0,
        // 10: Unused
        0,
        0,
        0,
        0,
        // 11: Erase chip
        flexspi_lut_seq(CMD_DDR, Octal, 0x60, CMD_DDR, Octal, 0x9F),
        0,
        0,
        0,
        // Remainder is unused
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ]
}

#[link_section = ".keystore"]
#[used]
static KEYSTORE: [u8; 2048] = [0; 2048];

const FCB_ADDRESS: u32 = 0x0000_0400;
const FCB_SIZE: usize = 512;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_imxrt::init(Default::default());

    // NOTE: Make sure to adjust the alignment for the actual flash chip you are using.
    let alignment = FlashAlignment {
        read_alignment: 2,
        write_alignment: 2,
        sector_size: 4096,
        block_size: 64 * 1024,
        page_size: 256,
    };

    // Create a new FlexSPI NOR flash driver.
    // NOTE: This relies of the FlexSPI having been configured already by having a valid FCB in the flash memory.
    let mut flash = unsafe { FlexSpiNorFlash::new_unchecked(p.FLEXSPI, alignment) };

    defmt::info!("Reading flash ID");
    match flash.read_id() {
        Ok(id) => defmt::info!("Flash ID: 0x{:02X}", id),
        Err(e) => defmt::error!("Failed to read flash ID: {}", e),
    }

    // Read the FCB.
    let mut fcb = [0; FCB_SIZE];
    match flash.read(FCB_ADDRESS, &mut fcb) {
        Ok(()) => defmt::info!("FCB: {:02X}", fcb),
        Err(e) => defmt::error!("Failed to read FCB from flash: {}", e),
    }

    defmt::info!("Reading flash ID");
    match flash.read_id() {
        Ok(id) => defmt::info!("Flash ID: 0x{:02X}", id),
        Err(e) => defmt::error!("Failed to read flash ID: {}", e),
    }


    defmt::info!("Erasing sector");
    match flash.erase_sector(0x3FFF000) {
        Ok(()) => defmt::info!("Erased sector"),
        Err(e) => defmt::error!("Failed to erase sector: {}", e),
    }
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    defmt::error!("Program panicked");
    loop {
        cortex_m::asm::wfe();
    }
}
