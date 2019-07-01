MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 16K
  RAM : ORIGIN = 0x10000000, LENGTH = 4K
}

PROVIDE(SPI0 = DefaultHandler);
PROVIDE(SPI1 = DefaultHandler);
PROVIDE(UART0 = DefaultHandler);
PROVIDE(UART1 = DefaultHandler);
PROVIDE(UART2 = DefaultHandler);
PROVIDE(I2C = DefaultHandler);
PROVIDE(SCT = DefaultHandler);
PROVIDE(MRT = DefaultHandler);
PROVIDE(CMP = DefaultHandler);
PROVIDE(WDT = DefaultHandler);
PROVIDE(BOD = DefaultHandler);
PROVIDE(FLASH_IRQ = DefaultHandler);
PROVIDE(WKT = DefaultHandler);
PROVIDE(PININT0 = DefaultHandler);
PROVIDE(PININT1 = DefaultHandler);
PROVIDE(PININT2 = DefaultHandler);
PROVIDE(PININT3 = DefaultHandler);
PROVIDE(PININT4 = DefaultHandler);
PROVIDE(PININT5 = DefaultHandler);
PROVIDE(PININT6 = DefaultHandler);
PROVIDE(PININT7 = DefaultHandler);
