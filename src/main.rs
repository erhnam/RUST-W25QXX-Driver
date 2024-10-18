use std::collections::HashMap;
use anyhow::Ok;
use linux_embedded_hal::spidev::{Spidev, SpiModeFlags, SpidevOptions};
use linux_embedded_hal::sysfs_gpio::Direction;
use linux_embedded_hal::SysfsPin;

mod w25qxx;
use w25qxx::W25qxx;

const W25QXX_HZ: u32 = 10_000_000;

fn gpio_get_pin(pin_num: u64) -> u64 {
    let pin_map: HashMap<u64, u64> = [
        (1, 508),
        (2, 509),
        (4, 378),
        (5, 377),
        (6, 371),
        (7, 372),
        (9, 375),
        (10, 374),
        (11, 373),
        (12, 370),
        (14, 425),
        (15, 426),
        (16, 496),
        (17, 497),
        (19, 494),
        (20, 495),
        (21, 503),
        (22, 504),
        (24, 502),
        (25, 505),
        (26, 507),
        (27, 506),
        (29, 356),
        (41, 440),
    ]
    .iter()
    .cloned()
    .collect();

    *pin_map.get(&pin_num).unwrap_or(&0)
}

fn main() -> anyhow::Result<()> {
    let spi_flash_cs = SysfsPin::new(gpio_get_pin(22));
    spi_flash_cs.export().unwrap();
    while !spi_flash_cs.is_exported() {}
    spi_flash_cs.set_direction(Direction::Out).unwrap();
    spi_flash_cs.set_value(1).unwrap();

    let mut spi1 = Spidev::open("/dev/spidev0.0")?;
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(W25QXX_HZ)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi1.configure(&options)?;

    let mut flash_memory: W25qxx<Spidev, SysfsPin> = W25qxx::new(spi1, spi_flash_cs).expect("Error to initializate interface SPI");

    // Ahora puedes continuar con el uso de `flash`
    if let Err(e) = flash_memory.init() {
        eprintln!("Error Initialize: {:?}", e);
        return Err(anyhow::Error::msg("Initialization failed"));
    }

    // Direccion y datos de ejemplo para escribir y leer
    let address: u32 = 0x00000000;
    let mut data_to_write: [u8; 64] = [0x00; 64]; // Datos a escribir, tamaño depende de la página
    let mut read_buffer: [u8; 64] = [0x00; 64];  // Buffer para almacenar datos leídos

    // Fill Write Buffer
    for i in 0..data_to_write.len() {
        data_to_write[i] = i as u8; // Rellena con valores de 0 a 15
    }

    println!("Erase Data {:?}:\n", 4096);

    let _ = flash_memory.erase(address,4096);

    println!("Write Data: {:?}\n", data_to_write);

    let _ = flash_memory.write(address, &mut data_to_write);

    let _ = flash_memory.read(address, &mut read_buffer);

    println!("Read Data: {:?}\n", read_buffer);

    Ok(())
}
