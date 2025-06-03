#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    maccr: Maccr,
    macecr: Macecr,
    macpfr: Macpfr,
    macwtr: Macwtr,
    macht0r: Macht0r,
    macht1r: Macht1r,
    _reserved6: [u8; 0x38],
    macvtr: Macvtr,
    _reserved7: [u8; 0x04],
    macvhtr: Macvhtr,
    _reserved8: [u8; 0x04],
    macvir: Macvir,
    macivir: Macivir,
    _reserved10: [u8; 0x08],
    macqtx_fcr: MacqtxFcr,
    _reserved11: [u8; 0x1c],
    macrx_fcr: MacrxFcr,
    _reserved12: [u8; 0x1c],
    macisr: Macisr,
    macier: Macier,
    macrx_tx_sr: MacrxTxSr,
    _reserved15: [u8; 0x04],
    macpcsr: Macpcsr,
    macrwkpfr: Macrwkpfr,
    _reserved17: [u8; 0x08],
    maclcsr: Maclcsr,
    macltcr: Macltcr,
    macletr: Macletr,
    mac1ustcr: Mac1ustcr,
    _reserved21: [u8; 0x30],
    macvr: Macvr,
    macdr: Macdr,
    _reserved23: [u8; 0x08],
    machwf1r: Machwf1r,
    machwf2r: Machwf2r,
    _reserved25: [u8; 0xd8],
    macmdioar: Macmdioar,
    macmdiodr: Macmdiodr,
    _reserved27: [u8; 0xf8],
    maca0hr: Maca0hr,
    maca0lr: Maca0lr,
    maca1hr: Maca1hr,
    maca1lr: Maca1lr,
    maca2hr: Maca2hr,
    maca2lr: Maca2lr,
    maca3hr: Maca3hr,
    maca3lr: Maca3lr,
    _reserved35: [u8; 0x03e0],
    mmc_control: MmcControl,
    mmc_rx_interrupt: MmcRxInterrupt,
    mmc_tx_interrupt: MmcTxInterrupt,
    mmc_rx_interrupt_mask: MmcRxInterruptMask,
    mmc_tx_interrupt_mask: MmcTxInterruptMask,
    _reserved40: [u8; 0x38],
    tx_single_collision_good_packets: TxSingleCollisionGoodPackets,
    tx_multiple_collision_good_packets: TxMultipleCollisionGoodPackets,
    _reserved42: [u8; 0x14],
    tx_packet_count_good: TxPacketCountGood,
    _reserved43: [u8; 0x28],
    rx_crc_error_packets: RxCrcErrorPackets,
    rx_alignment_error_packets: RxAlignmentErrorPackets,
    _reserved45: [u8; 0x28],
    rx_unicast_packets_good: RxUnicastPacketsGood,
    _reserved46: [u8; 0x24],
    tx_lpi_usec_cntr: TxLpiUsecCntr,
    tx_lpi_tran_cntr: TxLpiTranCntr,
    rx_lpi_usec_cntr: RxLpiUsecCntr,
    rx_lpi_tran_cntr: RxLpiTranCntr,
    _reserved50: [u8; 0x0104],
    macl3l4c0r: Macl3l4c0r,
    macl4a0r: Macl4a0r,
    _reserved52: [u8; 0x08],
    macl3a00r: Macl3a00r,
    macl3a10r: Macl3a10r,
    macl3a20: Macl3a20,
    macl3a30: Macl3a30,
    _reserved56: [u8; 0x10],
    macl3l4c1r: Macl3l4c1r,
    macl4a1r: Macl4a1r,
    _reserved58: [u8; 0x08],
    macl3a01r: Macl3a01r,
    macl3a11r: Macl3a11r,
    macl3a21r: Macl3a21r,
    macl3a31r: Macl3a31r,
    _reserved62: [u8; 0x0190],
    macarpar: Macarpar,
    _reserved63: [u8; 0x1c],
    mactscr: Mactscr,
    macssir: Macssir,
    macstsr: Macstsr,
    macstnr: Macstnr,
    macstsur: Macstsur,
    macstnur: Macstnur,
    mactsar: Mactsar,
    _reserved70: [u8; 0x04],
    mactssr: Mactssr,
    _reserved71: [u8; 0x0c],
    mactx_tssnr: MactxTssnr,
    mactx_tsssr: MactxTsssr,
    _reserved73: [u8; 0x08],
    macacr: Macacr,
    _reserved74: [u8; 0x04],
    macatsnr: Macatsnr,
    macatssr: Macatssr,
    mactsiacr: Mactsiacr,
    mactseacr: Mactseacr,
    mactsicnr: Mactsicnr,
    mactsecnr: Mactsecnr,
    _reserved80: [u8; 0x10],
    macppscr: Macppscr,
    _reserved81: [u8; 0x0c],
    macppsttsr: Macppsttsr,
    macppsttnr: Macppsttnr,
    macppsir: Macppsir,
    macppswr: Macppswr,
    _reserved85: [u8; 0x30],
    macpocr: Macpocr,
    macspi0r: Macspi0r,
    macspi1r: Macspi1r,
    macspi2r: Macspi2r,
    maclmir: Maclmir,
    _reserved90: [u8; 0x2c],
    mtlomr: Mtlomr,
    _reserved91: [u8; 0x1c],
    mtlisr: Mtlisr,
    _reserved92: [u8; 0xdc],
    mtltx_qomr: MtltxQomr,
    mtltx_qur: MtltxQur,
    mtltx_qdr: MtltxQdr,
    _reserved95: [u8; 0x20],
    mtlqicsr: Mtlqicsr,
    mtlrx_qomr: MtlrxQomr,
    mtlrx_qmpocr: MtlrxQmpocr,
    mtlrx_qdr: MtlrxQdr,
    _reserved99: [u8; 0x02c4],
    dmamr: Dmamr,
    dmasbmr: Dmasbmr,
    dmaisr: Dmaisr,
    dmadsr: Dmadsr,
    _reserved103: [u8; 0xf0],
    dmaccr: Dmaccr,
    dmactx_cr: DmactxCr,
    dmacrx_cr: DmacrxCr,
    _reserved106: [u8; 0x08],
    dmactx_dlar: DmactxDlar,
    _reserved107: [u8; 0x04],
    dmacrx_dlar: DmacrxDlar,
    dmactx_dtpr: DmactxDtpr,
    _reserved109: [u8; 0x04],
    dmacrx_dtpr: DmacrxDtpr,
    dmactx_rlr: DmactxRlr,
    dmacrx_rlr: DmacrxRlr,
    dmacier: Dmacier,
    dmacrx_iwtr: DmacrxIwtr,
    _reserved114: [u8; 0x08],
    dmaccatx_dr: DmaccatxDr,
    _reserved115: [u8; 0x04],
    dmaccarx_dr: DmaccarxDr,
    _reserved116: [u8; 0x04],
    dmaccatx_br: DmaccatxBr,
    _reserved117: [u8; 0x04],
    dmaccarx_br: DmaccarxBr,
    dmacsr: Dmacsr,
    _reserved119: [u8; 0x08],
    dmacmfcr: Dmacmfcr,
}
impl RegisterBlock {
    #[doc = "0x00 - Operating mode configuration register"]
    #[inline(always)]
    pub const fn maccr(&self) -> &Maccr {
        &self.maccr
    }
    #[doc = "0x04 - Extended operating mode configuration register"]
    #[inline(always)]
    pub const fn macecr(&self) -> &Macecr {
        &self.macecr
    }
    #[doc = "0x08 - Packet filtering control register"]
    #[inline(always)]
    pub const fn macpfr(&self) -> &Macpfr {
        &self.macpfr
    }
    #[doc = "0x0c - Watchdog timeout register"]
    #[inline(always)]
    pub const fn macwtr(&self) -> &Macwtr {
        &self.macwtr
    }
    #[doc = "0x10 - Hash Table 0 register"]
    #[inline(always)]
    pub const fn macht0r(&self) -> &Macht0r {
        &self.macht0r
    }
    #[doc = "0x14 - Hash Table 1 register"]
    #[inline(always)]
    pub const fn macht1r(&self) -> &Macht1r {
        &self.macht1r
    }
    #[doc = "0x50 - VLAN tag register"]
    #[inline(always)]
    pub const fn macvtr(&self) -> &Macvtr {
        &self.macvtr
    }
    #[doc = "0x58 - VLAN Hash table register"]
    #[inline(always)]
    pub const fn macvhtr(&self) -> &Macvhtr {
        &self.macvhtr
    }
    #[doc = "0x60 - VLAN inclusion register"]
    #[inline(always)]
    pub const fn macvir(&self) -> &Macvir {
        &self.macvir
    }
    #[doc = "0x64 - Inner VLAN inclusion register"]
    #[inline(always)]
    pub const fn macivir(&self) -> &Macivir {
        &self.macivir
    }
    #[doc = "0x70 - Tx Queue flow control register"]
    #[inline(always)]
    pub const fn macqtx_fcr(&self) -> &MacqtxFcr {
        &self.macqtx_fcr
    }
    #[doc = "0x90 - Rx flow control register"]
    #[inline(always)]
    pub const fn macrx_fcr(&self) -> &MacrxFcr {
        &self.macrx_fcr
    }
    #[doc = "0xb0 - Interrupt status register"]
    #[inline(always)]
    pub const fn macisr(&self) -> &Macisr {
        &self.macisr
    }
    #[doc = "0xb4 - Interrupt enable register"]
    #[inline(always)]
    pub const fn macier(&self) -> &Macier {
        &self.macier
    }
    #[doc = "0xb8 - Rx Tx status register"]
    #[inline(always)]
    pub const fn macrx_tx_sr(&self) -> &MacrxTxSr {
        &self.macrx_tx_sr
    }
    #[doc = "0xc0 - PMT control status register"]
    #[inline(always)]
    pub const fn macpcsr(&self) -> &Macpcsr {
        &self.macpcsr
    }
    #[doc = "0xc4 - Remove wakeup packet filter register"]
    #[inline(always)]
    pub const fn macrwkpfr(&self) -> &Macrwkpfr {
        &self.macrwkpfr
    }
    #[doc = "0xd0 - LPI control status register"]
    #[inline(always)]
    pub const fn maclcsr(&self) -> &Maclcsr {
        &self.maclcsr
    }
    #[doc = "0xd4 - LPI timers control register"]
    #[inline(always)]
    pub const fn macltcr(&self) -> &Macltcr {
        &self.macltcr
    }
    #[doc = "0xd8 - LPI entry timer register"]
    #[inline(always)]
    pub const fn macletr(&self) -> &Macletr {
        &self.macletr
    }
    #[doc = "0xdc - 1-microsecond-tick counter register"]
    #[inline(always)]
    pub const fn mac1ustcr(&self) -> &Mac1ustcr {
        &self.mac1ustcr
    }
    #[doc = "0x110 - Version register"]
    #[inline(always)]
    pub const fn macvr(&self) -> &Macvr {
        &self.macvr
    }
    #[doc = "0x114 - Debug register"]
    #[inline(always)]
    pub const fn macdr(&self) -> &Macdr {
        &self.macdr
    }
    #[doc = "0x120 - HW feature 1 register"]
    #[inline(always)]
    pub const fn machwf1r(&self) -> &Machwf1r {
        &self.machwf1r
    }
    #[doc = "0x124 - HW feature 2 register"]
    #[inline(always)]
    pub const fn machwf2r(&self) -> &Machwf2r {
        &self.machwf2r
    }
    #[doc = "0x200 - MDIO address register"]
    #[inline(always)]
    pub const fn macmdioar(&self) -> &Macmdioar {
        &self.macmdioar
    }
    #[doc = "0x204 - MDIO data register"]
    #[inline(always)]
    pub const fn macmdiodr(&self) -> &Macmdiodr {
        &self.macmdiodr
    }
    #[doc = "0x300 - Address 0 high register"]
    #[inline(always)]
    pub const fn maca0hr(&self) -> &Maca0hr {
        &self.maca0hr
    }
    #[doc = "0x304 - Address 0 low register"]
    #[inline(always)]
    pub const fn maca0lr(&self) -> &Maca0lr {
        &self.maca0lr
    }
    #[doc = "0x308 - Address 1 high register"]
    #[inline(always)]
    pub const fn maca1hr(&self) -> &Maca1hr {
        &self.maca1hr
    }
    #[doc = "0x30c - Address 1 low register"]
    #[inline(always)]
    pub const fn maca1lr(&self) -> &Maca1lr {
        &self.maca1lr
    }
    #[doc = "0x310 - Address 2 high register"]
    #[inline(always)]
    pub const fn maca2hr(&self) -> &Maca2hr {
        &self.maca2hr
    }
    #[doc = "0x314 - Address 2 low register"]
    #[inline(always)]
    pub const fn maca2lr(&self) -> &Maca2lr {
        &self.maca2lr
    }
    #[doc = "0x318 - Address 3 high register"]
    #[inline(always)]
    pub const fn maca3hr(&self) -> &Maca3hr {
        &self.maca3hr
    }
    #[doc = "0x31c - Address 3 low register"]
    #[inline(always)]
    pub const fn maca3lr(&self) -> &Maca3lr {
        &self.maca3lr
    }
    #[doc = "0x700 - MMC control register"]
    #[inline(always)]
    pub const fn mmc_control(&self) -> &MmcControl {
        &self.mmc_control
    }
    #[doc = "0x704 - MMC Rx interrupt register"]
    #[inline(always)]
    pub const fn mmc_rx_interrupt(&self) -> &MmcRxInterrupt {
        &self.mmc_rx_interrupt
    }
    #[doc = "0x708 - MMC Tx interrupt register"]
    #[inline(always)]
    pub const fn mmc_tx_interrupt(&self) -> &MmcTxInterrupt {
        &self.mmc_tx_interrupt
    }
    #[doc = "0x70c - MMC Rx interrupt mask register"]
    #[inline(always)]
    pub const fn mmc_rx_interrupt_mask(&self) -> &MmcRxInterruptMask {
        &self.mmc_rx_interrupt_mask
    }
    #[doc = "0x710 - MMC Tx interrupt mask register"]
    #[inline(always)]
    pub const fn mmc_tx_interrupt_mask(&self) -> &MmcTxInterruptMask {
        &self.mmc_tx_interrupt_mask
    }
    #[doc = "0x74c - Tx single collision good packets register"]
    #[inline(always)]
    pub const fn tx_single_collision_good_packets(&self) -> &TxSingleCollisionGoodPackets {
        &self.tx_single_collision_good_packets
    }
    #[doc = "0x750 - Tx multiple collision good packets register"]
    #[inline(always)]
    pub const fn tx_multiple_collision_good_packets(&self) -> &TxMultipleCollisionGoodPackets {
        &self.tx_multiple_collision_good_packets
    }
    #[doc = "0x768 - Tx packet count good register"]
    #[inline(always)]
    pub const fn tx_packet_count_good(&self) -> &TxPacketCountGood {
        &self.tx_packet_count_good
    }
    #[doc = "0x794 - Rx CRC error packets register"]
    #[inline(always)]
    pub const fn rx_crc_error_packets(&self) -> &RxCrcErrorPackets {
        &self.rx_crc_error_packets
    }
    #[doc = "0x798 - Rx alignment error packets register"]
    #[inline(always)]
    pub const fn rx_alignment_error_packets(&self) -> &RxAlignmentErrorPackets {
        &self.rx_alignment_error_packets
    }
    #[doc = "0x7c4 - Rx unicast packets good register"]
    #[inline(always)]
    pub const fn rx_unicast_packets_good(&self) -> &RxUnicastPacketsGood {
        &self.rx_unicast_packets_good
    }
    #[doc = "0x7ec - Tx LPI microsecond timer register"]
    #[inline(always)]
    pub const fn tx_lpi_usec_cntr(&self) -> &TxLpiUsecCntr {
        &self.tx_lpi_usec_cntr
    }
    #[doc = "0x7f0 - Tx LPI transition counter register"]
    #[inline(always)]
    pub const fn tx_lpi_tran_cntr(&self) -> &TxLpiTranCntr {
        &self.tx_lpi_tran_cntr
    }
    #[doc = "0x7f4 - Rx LPI microsecond counter register"]
    #[inline(always)]
    pub const fn rx_lpi_usec_cntr(&self) -> &RxLpiUsecCntr {
        &self.rx_lpi_usec_cntr
    }
    #[doc = "0x7f8 - Rx LPI transition counter register"]
    #[inline(always)]
    pub const fn rx_lpi_tran_cntr(&self) -> &RxLpiTranCntr {
        &self.rx_lpi_tran_cntr
    }
    #[doc = "0x900 - L3 and L4 control 0 register"]
    #[inline(always)]
    pub const fn macl3l4c0r(&self) -> &Macl3l4c0r {
        &self.macl3l4c0r
    }
    #[doc = "0x904 - Layer4 address filter 0 register"]
    #[inline(always)]
    pub const fn macl4a0r(&self) -> &Macl4a0r {
        &self.macl4a0r
    }
    #[doc = "0x910 - MACL3A00R"]
    #[inline(always)]
    pub const fn macl3a00r(&self) -> &Macl3a00r {
        &self.macl3a00r
    }
    #[doc = "0x914 - Layer3 address 1 filter 0 register"]
    #[inline(always)]
    pub const fn macl3a10r(&self) -> &Macl3a10r {
        &self.macl3a10r
    }
    #[doc = "0x918 - Layer3 Address 2 filter 0 register"]
    #[inline(always)]
    pub const fn macl3a20(&self) -> &Macl3a20 {
        &self.macl3a20
    }
    #[doc = "0x91c - Layer3 Address 3 filter 0 register"]
    #[inline(always)]
    pub const fn macl3a30(&self) -> &Macl3a30 {
        &self.macl3a30
    }
    #[doc = "0x930 - L3 and L4 control 1 register"]
    #[inline(always)]
    pub const fn macl3l4c1r(&self) -> &Macl3l4c1r {
        &self.macl3l4c1r
    }
    #[doc = "0x934 - Layer 4 address filter 1 register"]
    #[inline(always)]
    pub const fn macl4a1r(&self) -> &Macl4a1r {
        &self.macl4a1r
    }
    #[doc = "0x940 - Layer3 address 0 filter 1 Register"]
    #[inline(always)]
    pub const fn macl3a01r(&self) -> &Macl3a01r {
        &self.macl3a01r
    }
    #[doc = "0x944 - Layer3 address 1 filter 1 register"]
    #[inline(always)]
    pub const fn macl3a11r(&self) -> &Macl3a11r {
        &self.macl3a11r
    }
    #[doc = "0x948 - Layer3 address 2 filter 1 Register"]
    #[inline(always)]
    pub const fn macl3a21r(&self) -> &Macl3a21r {
        &self.macl3a21r
    }
    #[doc = "0x94c - Layer3 address 3 filter 1 register"]
    #[inline(always)]
    pub const fn macl3a31r(&self) -> &Macl3a31r {
        &self.macl3a31r
    }
    #[doc = "0xae0 - ARP address register"]
    #[inline(always)]
    pub const fn macarpar(&self) -> &Macarpar {
        &self.macarpar
    }
    #[doc = "0xb00 - Timestamp control Register"]
    #[inline(always)]
    pub const fn mactscr(&self) -> &Mactscr {
        &self.mactscr
    }
    #[doc = "0xb04 - Sub-second increment register"]
    #[inline(always)]
    pub const fn macssir(&self) -> &Macssir {
        &self.macssir
    }
    #[doc = "0xb08 - System time seconds register"]
    #[inline(always)]
    pub const fn macstsr(&self) -> &Macstsr {
        &self.macstsr
    }
    #[doc = "0xb0c - System time nanoseconds register"]
    #[inline(always)]
    pub const fn macstnr(&self) -> &Macstnr {
        &self.macstnr
    }
    #[doc = "0xb10 - System time seconds update register"]
    #[inline(always)]
    pub const fn macstsur(&self) -> &Macstsur {
        &self.macstsur
    }
    #[doc = "0xb14 - System time nanoseconds update register"]
    #[inline(always)]
    pub const fn macstnur(&self) -> &Macstnur {
        &self.macstnur
    }
    #[doc = "0xb18 - Timestamp addend register"]
    #[inline(always)]
    pub const fn mactsar(&self) -> &Mactsar {
        &self.mactsar
    }
    #[doc = "0xb20 - Timestamp status register"]
    #[inline(always)]
    pub const fn mactssr(&self) -> &Mactssr {
        &self.mactssr
    }
    #[doc = "0xb30 - Tx timestamp status nanoseconds register"]
    #[inline(always)]
    pub const fn mactx_tssnr(&self) -> &MactxTssnr {
        &self.mactx_tssnr
    }
    #[doc = "0xb34 - Tx timestamp status seconds register"]
    #[inline(always)]
    pub const fn mactx_tsssr(&self) -> &MactxTsssr {
        &self.mactx_tsssr
    }
    #[doc = "0xb40 - Auxiliary control register"]
    #[inline(always)]
    pub const fn macacr(&self) -> &Macacr {
        &self.macacr
    }
    #[doc = "0xb48 - Auxiliary timestamp nanoseconds register"]
    #[inline(always)]
    pub const fn macatsnr(&self) -> &Macatsnr {
        &self.macatsnr
    }
    #[doc = "0xb4c - Auxiliary timestamp seconds register"]
    #[inline(always)]
    pub const fn macatssr(&self) -> &Macatssr {
        &self.macatssr
    }
    #[doc = "0xb50 - Timestamp Ingress asymmetric correction register"]
    #[inline(always)]
    pub const fn mactsiacr(&self) -> &Mactsiacr {
        &self.mactsiacr
    }
    #[doc = "0xb54 - Timestamp Egress asymmetric correction register"]
    #[inline(always)]
    pub const fn mactseacr(&self) -> &Mactseacr {
        &self.mactseacr
    }
    #[doc = "0xb58 - Timestamp Ingress correction nanosecond register"]
    #[inline(always)]
    pub const fn mactsicnr(&self) -> &Mactsicnr {
        &self.mactsicnr
    }
    #[doc = "0xb5c - Timestamp Egress correction nanosecond register"]
    #[inline(always)]
    pub const fn mactsecnr(&self) -> &Mactsecnr {
        &self.mactsecnr
    }
    #[doc = "0xb70 - PPS control register"]
    #[inline(always)]
    pub const fn macppscr(&self) -> &Macppscr {
        &self.macppscr
    }
    #[doc = "0xb80 - PPS target time seconds register"]
    #[inline(always)]
    pub const fn macppsttsr(&self) -> &Macppsttsr {
        &self.macppsttsr
    }
    #[doc = "0xb84 - PPS target time nanoseconds register"]
    #[inline(always)]
    pub const fn macppsttnr(&self) -> &Macppsttnr {
        &self.macppsttnr
    }
    #[doc = "0xb88 - PPS interval register"]
    #[inline(always)]
    pub const fn macppsir(&self) -> &Macppsir {
        &self.macppsir
    }
    #[doc = "0xb8c - PPS width register"]
    #[inline(always)]
    pub const fn macppswr(&self) -> &Macppswr {
        &self.macppswr
    }
    #[doc = "0xbc0 - PTP Offload control register"]
    #[inline(always)]
    pub const fn macpocr(&self) -> &Macpocr {
        &self.macpocr
    }
    #[doc = "0xbc4 - PTP Source Port Identity 0 Register"]
    #[inline(always)]
    pub const fn macspi0r(&self) -> &Macspi0r {
        &self.macspi0r
    }
    #[doc = "0xbc8 - PTP Source port identity 1 register"]
    #[inline(always)]
    pub const fn macspi1r(&self) -> &Macspi1r {
        &self.macspi1r
    }
    #[doc = "0xbcc - PTP Source port identity 2 register"]
    #[inline(always)]
    pub const fn macspi2r(&self) -> &Macspi2r {
        &self.macspi2r
    }
    #[doc = "0xbd0 - Log message interval register"]
    #[inline(always)]
    pub const fn maclmir(&self) -> &Maclmir {
        &self.maclmir
    }
    #[doc = "0xc00 - Operating mode Register"]
    #[inline(always)]
    pub const fn mtlomr(&self) -> &Mtlomr {
        &self.mtlomr
    }
    #[doc = "0xc20 - Interrupt status Register"]
    #[inline(always)]
    pub const fn mtlisr(&self) -> &Mtlisr {
        &self.mtlisr
    }
    #[doc = "0xd00 - Tx queue operating mode Register"]
    #[inline(always)]
    pub const fn mtltx_qomr(&self) -> &MtltxQomr {
        &self.mtltx_qomr
    }
    #[doc = "0xd04 - Tx queue underflow register"]
    #[inline(always)]
    pub const fn mtltx_qur(&self) -> &MtltxQur {
        &self.mtltx_qur
    }
    #[doc = "0xd08 - Tx queue debug Register"]
    #[inline(always)]
    pub const fn mtltx_qdr(&self) -> &MtltxQdr {
        &self.mtltx_qdr
    }
    #[doc = "0xd2c - Queue interrupt control status Register"]
    #[inline(always)]
    pub const fn mtlqicsr(&self) -> &Mtlqicsr {
        &self.mtlqicsr
    }
    #[doc = "0xd30 - Rx queue operating mode register"]
    #[inline(always)]
    pub const fn mtlrx_qomr(&self) -> &MtlrxQomr {
        &self.mtlrx_qomr
    }
    #[doc = "0xd34 - Rx queue missed packet and overflow counter register"]
    #[inline(always)]
    pub const fn mtlrx_qmpocr(&self) -> &MtlrxQmpocr {
        &self.mtlrx_qmpocr
    }
    #[doc = "0xd38 - Rx queue debug register"]
    #[inline(always)]
    pub const fn mtlrx_qdr(&self) -> &MtlrxQdr {
        &self.mtlrx_qdr
    }
    #[doc = "0x1000 - DMA mode register"]
    #[inline(always)]
    pub const fn dmamr(&self) -> &Dmamr {
        &self.dmamr
    }
    #[doc = "0x1004 - System bus mode register"]
    #[inline(always)]
    pub const fn dmasbmr(&self) -> &Dmasbmr {
        &self.dmasbmr
    }
    #[doc = "0x1008 - Interrupt status register"]
    #[inline(always)]
    pub const fn dmaisr(&self) -> &Dmaisr {
        &self.dmaisr
    }
    #[doc = "0x100c - Debug status register"]
    #[inline(always)]
    pub const fn dmadsr(&self) -> &Dmadsr {
        &self.dmadsr
    }
    #[doc = "0x1100 - Channel control register"]
    #[inline(always)]
    pub const fn dmaccr(&self) -> &Dmaccr {
        &self.dmaccr
    }
    #[doc = "0x1104 - Channel transmit control register"]
    #[inline(always)]
    pub const fn dmactx_cr(&self) -> &DmactxCr {
        &self.dmactx_cr
    }
    #[doc = "0x1108 - Channel receive control register"]
    #[inline(always)]
    pub const fn dmacrx_cr(&self) -> &DmacrxCr {
        &self.dmacrx_cr
    }
    #[doc = "0x1114 - Channel Tx descriptor list address register"]
    #[inline(always)]
    pub const fn dmactx_dlar(&self) -> &DmactxDlar {
        &self.dmactx_dlar
    }
    #[doc = "0x111c - Channel Rx descriptor list address register"]
    #[inline(always)]
    pub const fn dmacrx_dlar(&self) -> &DmacrxDlar {
        &self.dmacrx_dlar
    }
    #[doc = "0x1120 - Channel Tx descriptor tail pointer register"]
    #[inline(always)]
    pub const fn dmactx_dtpr(&self) -> &DmactxDtpr {
        &self.dmactx_dtpr
    }
    #[doc = "0x1128 - Channel Rx descriptor tail pointer register"]
    #[inline(always)]
    pub const fn dmacrx_dtpr(&self) -> &DmacrxDtpr {
        &self.dmacrx_dtpr
    }
    #[doc = "0x112c - Channel Tx descriptor ring length register"]
    #[inline(always)]
    pub const fn dmactx_rlr(&self) -> &DmactxRlr {
        &self.dmactx_rlr
    }
    #[doc = "0x1130 - Channel Rx descriptor ring length register"]
    #[inline(always)]
    pub const fn dmacrx_rlr(&self) -> &DmacrxRlr {
        &self.dmacrx_rlr
    }
    #[doc = "0x1134 - Channel interrupt enable register"]
    #[inline(always)]
    pub const fn dmacier(&self) -> &Dmacier {
        &self.dmacier
    }
    #[doc = "0x1138 - Channel Rx interrupt watchdog timer register"]
    #[inline(always)]
    pub const fn dmacrx_iwtr(&self) -> &DmacrxIwtr {
        &self.dmacrx_iwtr
    }
    #[doc = "0x1144 - Channel current application transmit descriptor register"]
    #[inline(always)]
    pub const fn dmaccatx_dr(&self) -> &DmaccatxDr {
        &self.dmaccatx_dr
    }
    #[doc = "0x114c - Channel current application receive descriptor register"]
    #[inline(always)]
    pub const fn dmaccarx_dr(&self) -> &DmaccarxDr {
        &self.dmaccarx_dr
    }
    #[doc = "0x1154 - Channel current application transmit buffer register"]
    #[inline(always)]
    pub const fn dmaccatx_br(&self) -> &DmaccatxBr {
        &self.dmaccatx_br
    }
    #[doc = "0x115c - Channel current application receive buffer register"]
    #[inline(always)]
    pub const fn dmaccarx_br(&self) -> &DmaccarxBr {
        &self.dmaccarx_br
    }
    #[doc = "0x1160 - Channel status register"]
    #[inline(always)]
    pub const fn dmacsr(&self) -> &Dmacsr {
        &self.dmacsr
    }
    #[doc = "0x116c - Channel missed frame count register"]
    #[inline(always)]
    pub const fn dmacmfcr(&self) -> &Dmacmfcr {
        &self.dmacmfcr
    }
}
#[doc = "DMAMR (rw) register accessor: DMA mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmamr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamr`] module"]
#[doc(alias = "DMAMR")]
pub type Dmamr = crate::Reg<dmamr::DmamrSpec>;
#[doc = "DMA mode register"]
pub mod dmamr;
#[doc = "DMASBMR (rw) register accessor: System bus mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasbmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasbmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasbmr`] module"]
#[doc(alias = "DMASBMR")]
pub type Dmasbmr = crate::Reg<dmasbmr::DmasbmrSpec>;
#[doc = "System bus mode register"]
pub mod dmasbmr;
#[doc = "DMAISR (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaisr`] module"]
#[doc(alias = "DMAISR")]
pub type Dmaisr = crate::Reg<dmaisr::DmaisrSpec>;
#[doc = "Interrupt status register"]
pub mod dmaisr;
#[doc = "DMADSR (r) register accessor: Debug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmadsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmadsr`] module"]
#[doc(alias = "DMADSR")]
pub type Dmadsr = crate::Reg<dmadsr::DmadsrSpec>;
#[doc = "Debug status register"]
pub mod dmadsr;
#[doc = "DMACCR (rw) register accessor: Channel control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaccr`] module"]
#[doc(alias = "DMACCR")]
pub type Dmaccr = crate::Reg<dmaccr::DmaccrSpec>;
#[doc = "Channel control register"]
pub mod dmaccr;
#[doc = "DMACTxCR (rw) register accessor: Channel transmit control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactx_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactx_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactx_cr`] module"]
#[doc(alias = "DMACTxCR")]
pub type DmactxCr = crate::Reg<dmactx_cr::DmactxCrSpec>;
#[doc = "Channel transmit control register"]
pub mod dmactx_cr;
#[doc = "DMACRxCR (rw) register accessor: Channel receive control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacrx_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrx_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacrx_cr`] module"]
#[doc(alias = "DMACRxCR")]
pub type DmacrxCr = crate::Reg<dmacrx_cr::DmacrxCrSpec>;
#[doc = "Channel receive control register"]
pub mod dmacrx_cr;
#[doc = "DMACTxDLAR (rw) register accessor: Channel Tx descriptor list address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactx_dlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactx_dlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactx_dlar`] module"]
#[doc(alias = "DMACTxDLAR")]
pub type DmactxDlar = crate::Reg<dmactx_dlar::DmactxDlarSpec>;
#[doc = "Channel Tx descriptor list address register"]
pub mod dmactx_dlar;
#[doc = "DMACRxDLAR (rw) register accessor: Channel Rx descriptor list address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacrx_dlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrx_dlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacrx_dlar`] module"]
#[doc(alias = "DMACRxDLAR")]
pub type DmacrxDlar = crate::Reg<dmacrx_dlar::DmacrxDlarSpec>;
#[doc = "Channel Rx descriptor list address register"]
pub mod dmacrx_dlar;
#[doc = "DMACTxDTPR (rw) register accessor: Channel Tx descriptor tail pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactx_dtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactx_dtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactx_dtpr`] module"]
#[doc(alias = "DMACTxDTPR")]
pub type DmactxDtpr = crate::Reg<dmactx_dtpr::DmactxDtprSpec>;
#[doc = "Channel Tx descriptor tail pointer register"]
pub mod dmactx_dtpr;
#[doc = "DMACRxDTPR (rw) register accessor: Channel Rx descriptor tail pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacrx_dtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrx_dtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacrx_dtpr`] module"]
#[doc(alias = "DMACRxDTPR")]
pub type DmacrxDtpr = crate::Reg<dmacrx_dtpr::DmacrxDtprSpec>;
#[doc = "Channel Rx descriptor tail pointer register"]
pub mod dmacrx_dtpr;
#[doc = "DMACTxRLR (rw) register accessor: Channel Tx descriptor ring length register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactx_rlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactx_rlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactx_rlr`] module"]
#[doc(alias = "DMACTxRLR")]
pub type DmactxRlr = crate::Reg<dmactx_rlr::DmactxRlrSpec>;
#[doc = "Channel Tx descriptor ring length register"]
pub mod dmactx_rlr;
#[doc = "DMACRxRLR (rw) register accessor: Channel Rx descriptor ring length register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacrx_rlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrx_rlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacrx_rlr`] module"]
#[doc(alias = "DMACRxRLR")]
pub type DmacrxRlr = crate::Reg<dmacrx_rlr::DmacrxRlrSpec>;
#[doc = "Channel Rx descriptor ring length register"]
pub mod dmacrx_rlr;
#[doc = "DMACIER (rw) register accessor: Channel interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacier`] module"]
#[doc(alias = "DMACIER")]
pub type Dmacier = crate::Reg<dmacier::DmacierSpec>;
#[doc = "Channel interrupt enable register"]
pub mod dmacier;
#[doc = "DMACRxIWTR (rw) register accessor: Channel Rx interrupt watchdog timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacrx_iwtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrx_iwtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacrx_iwtr`] module"]
#[doc(alias = "DMACRxIWTR")]
pub type DmacrxIwtr = crate::Reg<dmacrx_iwtr::DmacrxIwtrSpec>;
#[doc = "Channel Rx interrupt watchdog timer register"]
pub mod dmacrx_iwtr;
#[doc = "DMACCATxDR (r) register accessor: Channel current application transmit descriptor register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaccatx_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaccatx_dr`] module"]
#[doc(alias = "DMACCATxDR")]
pub type DmaccatxDr = crate::Reg<dmaccatx_dr::DmaccatxDrSpec>;
#[doc = "Channel current application transmit descriptor register"]
pub mod dmaccatx_dr;
#[doc = "DMACCARxDR (r) register accessor: Channel current application receive descriptor register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaccarx_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaccarx_dr`] module"]
#[doc(alias = "DMACCARxDR")]
pub type DmaccarxDr = crate::Reg<dmaccarx_dr::DmaccarxDrSpec>;
#[doc = "Channel current application receive descriptor register"]
pub mod dmaccarx_dr;
#[doc = "DMACCATxBR (r) register accessor: Channel current application transmit buffer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaccatx_br::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaccatx_br`] module"]
#[doc(alias = "DMACCATxBR")]
pub type DmaccatxBr = crate::Reg<dmaccatx_br::DmaccatxBrSpec>;
#[doc = "Channel current application transmit buffer register"]
pub mod dmaccatx_br;
#[doc = "DMACCARxBR (r) register accessor: Channel current application receive buffer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaccarx_br::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaccarx_br`] module"]
#[doc(alias = "DMACCARxBR")]
pub type DmaccarxBr = crate::Reg<dmaccarx_br::DmaccarxBrSpec>;
#[doc = "Channel current application receive buffer register"]
pub mod dmaccarx_br;
#[doc = "DMACSR (rw) register accessor: Channel status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacsr`] module"]
#[doc(alias = "DMACSR")]
pub type Dmacsr = crate::Reg<dmacsr::DmacsrSpec>;
#[doc = "Channel status register"]
pub mod dmacsr;
#[doc = "DMACMFCR (r) register accessor: Channel missed frame count register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacmfcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacmfcr`] module"]
#[doc(alias = "DMACMFCR")]
pub type Dmacmfcr = crate::Reg<dmacmfcr::DmacmfcrSpec>;
#[doc = "Channel missed frame count register"]
pub mod dmacmfcr;
#[doc = "MTLOMR (rw) register accessor: Operating mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtlomr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlomr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtlomr`] module"]
#[doc(alias = "MTLOMR")]
pub type Mtlomr = crate::Reg<mtlomr::MtlomrSpec>;
#[doc = "Operating mode Register"]
pub mod mtlomr;
#[doc = "MTLISR (r) register accessor: Interrupt status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtlisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtlisr`] module"]
#[doc(alias = "MTLISR")]
pub type Mtlisr = crate::Reg<mtlisr::MtlisrSpec>;
#[doc = "Interrupt status Register"]
pub mod mtlisr;
#[doc = "MTLTxQOMR (rw) register accessor: Tx queue operating mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtltx_qomr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltx_qomr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtltx_qomr`] module"]
#[doc(alias = "MTLTxQOMR")]
pub type MtltxQomr = crate::Reg<mtltx_qomr::MtltxQomrSpec>;
#[doc = "Tx queue operating mode Register"]
pub mod mtltx_qomr;
#[doc = "MTLTxQUR (r) register accessor: Tx queue underflow register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtltx_qur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtltx_qur`] module"]
#[doc(alias = "MTLTxQUR")]
pub type MtltxQur = crate::Reg<mtltx_qur::MtltxQurSpec>;
#[doc = "Tx queue underflow register"]
pub mod mtltx_qur;
#[doc = "MTLTxQDR (r) register accessor: Tx queue debug Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtltx_qdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtltx_qdr`] module"]
#[doc(alias = "MTLTxQDR")]
pub type MtltxQdr = crate::Reg<mtltx_qdr::MtltxQdrSpec>;
#[doc = "Tx queue debug Register"]
pub mod mtltx_qdr;
#[doc = "MTLQICSR (rw) register accessor: Queue interrupt control status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtlqicsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlqicsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtlqicsr`] module"]
#[doc(alias = "MTLQICSR")]
pub type Mtlqicsr = crate::Reg<mtlqicsr::MtlqicsrSpec>;
#[doc = "Queue interrupt control status Register"]
pub mod mtlqicsr;
#[doc = "MTLRxQOMR (rw) register accessor: Rx queue operating mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtlrx_qomr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrx_qomr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtlrx_qomr`] module"]
#[doc(alias = "MTLRxQOMR")]
pub type MtlrxQomr = crate::Reg<mtlrx_qomr::MtlrxQomrSpec>;
#[doc = "Rx queue operating mode register"]
pub mod mtlrx_qomr;
#[doc = "MTLRxQMPOCR (r) register accessor: Rx queue missed packet and overflow counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtlrx_qmpocr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtlrx_qmpocr`] module"]
#[doc(alias = "MTLRxQMPOCR")]
pub type MtlrxQmpocr = crate::Reg<mtlrx_qmpocr::MtlrxQmpocrSpec>;
#[doc = "Rx queue missed packet and overflow counter register"]
pub mod mtlrx_qmpocr;
#[doc = "MTLRxQDR (r) register accessor: Rx queue debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtlrx_qdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtlrx_qdr`] module"]
#[doc(alias = "MTLRxQDR")]
pub type MtlrxQdr = crate::Reg<mtlrx_qdr::MtlrxQdrSpec>;
#[doc = "Rx queue debug register"]
pub mod mtlrx_qdr;
#[doc = "MACCR (rw) register accessor: Operating mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`maccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maccr`] module"]
#[doc(alias = "MACCR")]
pub type Maccr = crate::Reg<maccr::MaccrSpec>;
#[doc = "Operating mode configuration register"]
pub mod maccr;
#[doc = "MACECR (rw) register accessor: Extended operating mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`macecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macecr`] module"]
#[doc(alias = "MACECR")]
pub type Macecr = crate::Reg<macecr::MacecrSpec>;
#[doc = "Extended operating mode configuration register"]
pub mod macecr;
#[doc = "MACPFR (rw) register accessor: Packet filtering control register\n\nYou can [`read`](crate::Reg::read) this register and get [`macpfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macpfr`] module"]
#[doc(alias = "MACPFR")]
pub type Macpfr = crate::Reg<macpfr::MacpfrSpec>;
#[doc = "Packet filtering control register"]
pub mod macpfr;
#[doc = "MACWTR (rw) register accessor: Watchdog timeout register\n\nYou can [`read`](crate::Reg::read) this register and get [`macwtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macwtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macwtr`] module"]
#[doc(alias = "MACWTR")]
pub type Macwtr = crate::Reg<macwtr::MacwtrSpec>;
#[doc = "Watchdog timeout register"]
pub mod macwtr;
#[doc = "MACHT0R (rw) register accessor: Hash Table 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macht0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macht0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macht0r`] module"]
#[doc(alias = "MACHT0R")]
pub type Macht0r = crate::Reg<macht0r::Macht0rSpec>;
#[doc = "Hash Table 0 register"]
pub mod macht0r;
#[doc = "MACHT1R (rw) register accessor: Hash Table 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macht1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macht1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macht1r`] module"]
#[doc(alias = "MACHT1R")]
pub type Macht1r = crate::Reg<macht1r::Macht1rSpec>;
#[doc = "Hash Table 1 register"]
pub mod macht1r;
#[doc = "MACVTR (rw) register accessor: VLAN tag register\n\nYou can [`read`](crate::Reg::read) this register and get [`macvtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macvtr`] module"]
#[doc(alias = "MACVTR")]
pub type Macvtr = crate::Reg<macvtr::MacvtrSpec>;
#[doc = "VLAN tag register"]
pub mod macvtr;
#[doc = "MACVHTR (rw) register accessor: VLAN Hash table register\n\nYou can [`read`](crate::Reg::read) this register and get [`macvhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macvhtr`] module"]
#[doc(alias = "MACVHTR")]
pub type Macvhtr = crate::Reg<macvhtr::MacvhtrSpec>;
#[doc = "VLAN Hash table register"]
pub mod macvhtr;
#[doc = "MACVIR (rw) register accessor: VLAN inclusion register\n\nYou can [`read`](crate::Reg::read) this register and get [`macvir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macvir`] module"]
#[doc(alias = "MACVIR")]
pub type Macvir = crate::Reg<macvir::MacvirSpec>;
#[doc = "VLAN inclusion register"]
pub mod macvir;
#[doc = "MACIVIR (rw) register accessor: Inner VLAN inclusion register\n\nYou can [`read`](crate::Reg::read) this register and get [`macivir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macivir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macivir`] module"]
#[doc(alias = "MACIVIR")]
pub type Macivir = crate::Reg<macivir::MacivirSpec>;
#[doc = "Inner VLAN inclusion register"]
pub mod macivir;
#[doc = "MACQTxFCR (rw) register accessor: Tx Queue flow control register\n\nYou can [`read`](crate::Reg::read) this register and get [`macqtx_fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macqtx_fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macqtx_fcr`] module"]
#[doc(alias = "MACQTxFCR")]
pub type MacqtxFcr = crate::Reg<macqtx_fcr::MacqtxFcrSpec>;
#[doc = "Tx Queue flow control register"]
pub mod macqtx_fcr;
#[doc = "MACRxFCR (rw) register accessor: Rx flow control register\n\nYou can [`read`](crate::Reg::read) this register and get [`macrx_fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrx_fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macrx_fcr`] module"]
#[doc(alias = "MACRxFCR")]
pub type MacrxFcr = crate::Reg<macrx_fcr::MacrxFcrSpec>;
#[doc = "Rx flow control register"]
pub mod macrx_fcr;
#[doc = "MACISR (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`macisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macisr`] module"]
#[doc(alias = "MACISR")]
pub type Macisr = crate::Reg<macisr::MacisrSpec>;
#[doc = "Interrupt status register"]
pub mod macisr;
#[doc = "MACIER (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`macier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macier`] module"]
#[doc(alias = "MACIER")]
pub type Macier = crate::Reg<macier::MacierSpec>;
#[doc = "Interrupt enable register"]
pub mod macier;
#[doc = "MACRxTxSR (r) register accessor: Rx Tx status register\n\nYou can [`read`](crate::Reg::read) this register and get [`macrx_tx_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macrx_tx_sr`] module"]
#[doc(alias = "MACRxTxSR")]
pub type MacrxTxSr = crate::Reg<macrx_tx_sr::MacrxTxSrSpec>;
#[doc = "Rx Tx status register"]
pub mod macrx_tx_sr;
#[doc = "MACPCSR (rw) register accessor: PMT control status register\n\nYou can [`read`](crate::Reg::read) this register and get [`macpcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macpcsr`] module"]
#[doc(alias = "MACPCSR")]
pub type Macpcsr = crate::Reg<macpcsr::MacpcsrSpec>;
#[doc = "PMT control status register"]
pub mod macpcsr;
#[doc = "MACRWKPFR (rw) register accessor: Remove wakeup packet filter register\n\nYou can [`read`](crate::Reg::read) this register and get [`macrwkpfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrwkpfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macrwkpfr`] module"]
#[doc(alias = "MACRWKPFR")]
pub type Macrwkpfr = crate::Reg<macrwkpfr::MacrwkpfrSpec>;
#[doc = "Remove wakeup packet filter register"]
pub mod macrwkpfr;
#[doc = "MACLCSR (rw) register accessor: LPI control status register\n\nYou can [`read`](crate::Reg::read) this register and get [`maclcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maclcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maclcsr`] module"]
#[doc(alias = "MACLCSR")]
pub type Maclcsr = crate::Reg<maclcsr::MaclcsrSpec>;
#[doc = "LPI control status register"]
pub mod maclcsr;
#[doc = "MACLTCR (rw) register accessor: LPI timers control register\n\nYou can [`read`](crate::Reg::read) this register and get [`macltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macltcr`] module"]
#[doc(alias = "MACLTCR")]
pub type Macltcr = crate::Reg<macltcr::MacltcrSpec>;
#[doc = "LPI timers control register"]
pub mod macltcr;
#[doc = "MACLETR (rw) register accessor: LPI entry timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`macletr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macletr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macletr`] module"]
#[doc(alias = "MACLETR")]
pub type Macletr = crate::Reg<macletr::MacletrSpec>;
#[doc = "LPI entry timer register"]
pub mod macletr;
#[doc = "MAC1USTCR (rw) register accessor: 1-microsecond-tick counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`mac1ustcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac1ustcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac1ustcr`] module"]
#[doc(alias = "MAC1USTCR")]
pub type Mac1ustcr = crate::Reg<mac1ustcr::Mac1ustcrSpec>;
#[doc = "1-microsecond-tick counter register"]
pub mod mac1ustcr;
#[doc = "MACVR (r) register accessor: Version register\n\nYou can [`read`](crate::Reg::read) this register and get [`macvr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macvr`] module"]
#[doc(alias = "MACVR")]
pub type Macvr = crate::Reg<macvr::MacvrSpec>;
#[doc = "Version register"]
pub mod macvr;
#[doc = "MACHWF1R (r) register accessor: HW feature 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`machwf1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@machwf1r`] module"]
#[doc(alias = "MACHWF1R")]
pub type Machwf1r = crate::Reg<machwf1r::Machwf1rSpec>;
#[doc = "HW feature 1 register"]
pub mod machwf1r;
#[doc = "MACHWF2R (r) register accessor: HW feature 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`machwf2r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@machwf2r`] module"]
#[doc(alias = "MACHWF2R")]
pub type Machwf2r = crate::Reg<machwf2r::Machwf2rSpec>;
#[doc = "HW feature 2 register"]
pub mod machwf2r;
#[doc = "MACMDIOAR (rw) register accessor: MDIO address register\n\nYou can [`read`](crate::Reg::read) this register and get [`macmdioar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmdioar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macmdioar`] module"]
#[doc(alias = "MACMDIOAR")]
pub type Macmdioar = crate::Reg<macmdioar::MacmdioarSpec>;
#[doc = "MDIO address register"]
pub mod macmdioar;
#[doc = "MACMDIODR (rw) register accessor: MDIO data register\n\nYou can [`read`](crate::Reg::read) this register and get [`macmdiodr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmdiodr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macmdiodr`] module"]
#[doc(alias = "MACMDIODR")]
pub type Macmdiodr = crate::Reg<macmdiodr::MacmdiodrSpec>;
#[doc = "MDIO data register"]
pub mod macmdiodr;
#[doc = "MACARPAR (rw) register accessor: ARP address register\n\nYou can [`read`](crate::Reg::read) this register and get [`macarpar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macarpar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macarpar`] module"]
#[doc(alias = "MACARPAR")]
pub type Macarpar = crate::Reg<macarpar::MacarparSpec>;
#[doc = "ARP address register"]
pub mod macarpar;
#[doc = "MACA0HR (rw) register accessor: Address 0 high register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca0hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca0hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca0hr`] module"]
#[doc(alias = "MACA0HR")]
pub type Maca0hr = crate::Reg<maca0hr::Maca0hrSpec>;
#[doc = "Address 0 high register"]
pub mod maca0hr;
#[doc = "MACA0LR (rw) register accessor: Address 0 low register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca0lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca0lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca0lr`] module"]
#[doc(alias = "MACA0LR")]
pub type Maca0lr = crate::Reg<maca0lr::Maca0lrSpec>;
#[doc = "Address 0 low register"]
pub mod maca0lr;
#[doc = "MACA1LR (rw) register accessor: Address 1 low register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca1lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca1lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca1lr`] module"]
#[doc(alias = "MACA1LR")]
pub type Maca1lr = crate::Reg<maca1lr::Maca1lrSpec>;
#[doc = "Address 1 low register"]
pub mod maca1lr;
#[doc = "MACA2LR (rw) register accessor: Address 2 low register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca2lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca2lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca2lr`] module"]
#[doc(alias = "MACA2LR")]
pub type Maca2lr = crate::Reg<maca2lr::Maca2lrSpec>;
#[doc = "Address 2 low register"]
pub mod maca2lr;
#[doc = "MACA3LR (rw) register accessor: Address 3 low register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca3lr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca3lr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca3lr`] module"]
#[doc(alias = "MACA3LR")]
pub type Maca3lr = crate::Reg<maca3lr::Maca3lrSpec>;
#[doc = "Address 3 low register"]
pub mod maca3lr;
#[doc = "MACA1HR (rw) register accessor: Address 1 high register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca1hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca1hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca1hr`] module"]
#[doc(alias = "MACA1HR")]
pub type Maca1hr = crate::Reg<maca1hr::Maca1hrSpec>;
#[doc = "Address 1 high register"]
pub mod maca1hr;
#[doc = "MACA2HR (rw) register accessor: Address 2 high register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca2hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca2hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca2hr`] module"]
#[doc(alias = "MACA2HR")]
pub type Maca2hr = crate::Reg<maca2hr::Maca2hrSpec>;
#[doc = "Address 2 high register"]
pub mod maca2hr;
#[doc = "MACA3HR (rw) register accessor: Address 3 high register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca3hr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca3hr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca3hr`] module"]
#[doc(alias = "MACA3HR")]
pub type Maca3hr = crate::Reg<maca3hr::Maca3hrSpec>;
#[doc = "Address 3 high register"]
pub mod maca3hr;
#[doc = "MMC_CONTROL (rw) register accessor: MMC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_control`] module"]
#[doc(alias = "MMC_CONTROL")]
pub type MmcControl = crate::Reg<mmc_control::MmcControlSpec>;
#[doc = "MMC control register"]
pub mod mmc_control;
#[doc = "MMC_RX_INTERRUPT (r) register accessor: MMC Rx interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_rx_interrupt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rx_interrupt`] module"]
#[doc(alias = "MMC_RX_INTERRUPT")]
pub type MmcRxInterrupt = crate::Reg<mmc_rx_interrupt::MmcRxInterruptSpec>;
#[doc = "MMC Rx interrupt register"]
pub mod mmc_rx_interrupt;
#[doc = "MMC_TX_INTERRUPT (r) register accessor: MMC Tx interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_tx_interrupt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_tx_interrupt`] module"]
#[doc(alias = "MMC_TX_INTERRUPT")]
pub type MmcTxInterrupt = crate::Reg<mmc_tx_interrupt::MmcTxInterruptSpec>;
#[doc = "MMC Tx interrupt register"]
pub mod mmc_tx_interrupt;
#[doc = "MMC_RX_INTERRUPT_MASK (rw) register accessor: MMC Rx interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_rx_interrupt_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_rx_interrupt_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rx_interrupt_mask`] module"]
#[doc(alias = "MMC_RX_INTERRUPT_MASK")]
pub type MmcRxInterruptMask = crate::Reg<mmc_rx_interrupt_mask::MmcRxInterruptMaskSpec>;
#[doc = "MMC Rx interrupt mask register"]
pub mod mmc_rx_interrupt_mask;
#[doc = "MMC_TX_INTERRUPT_MASK (rw) register accessor: MMC Tx interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_tx_interrupt_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_tx_interrupt_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_tx_interrupt_mask`] module"]
#[doc(alias = "MMC_TX_INTERRUPT_MASK")]
pub type MmcTxInterruptMask = crate::Reg<mmc_tx_interrupt_mask::MmcTxInterruptMaskSpec>;
#[doc = "MMC Tx interrupt mask register"]
pub mod mmc_tx_interrupt_mask;
#[doc = "TX_SINGLE_COLLISION_GOOD_PACKETS (r) register accessor: Tx single collision good packets register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_single_collision_good_packets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_single_collision_good_packets`] module"]
#[doc(alias = "TX_SINGLE_COLLISION_GOOD_PACKETS")]
pub type TxSingleCollisionGoodPackets =
    crate::Reg<tx_single_collision_good_packets::TxSingleCollisionGoodPacketsSpec>;
#[doc = "Tx single collision good packets register"]
pub mod tx_single_collision_good_packets;
#[doc = "TX_MULTIPLE_COLLISION_GOOD_PACKETS (r) register accessor: Tx multiple collision good packets register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_multiple_collision_good_packets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_multiple_collision_good_packets`] module"]
#[doc(alias = "TX_MULTIPLE_COLLISION_GOOD_PACKETS")]
pub type TxMultipleCollisionGoodPackets =
    crate::Reg<tx_multiple_collision_good_packets::TxMultipleCollisionGoodPacketsSpec>;
#[doc = "Tx multiple collision good packets register"]
pub mod tx_multiple_collision_good_packets;
#[doc = "TX_PACKET_COUNT_GOOD (r) register accessor: Tx packet count good register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_packet_count_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_packet_count_good`] module"]
#[doc(alias = "TX_PACKET_COUNT_GOOD")]
pub type TxPacketCountGood = crate::Reg<tx_packet_count_good::TxPacketCountGoodSpec>;
#[doc = "Tx packet count good register"]
pub mod tx_packet_count_good;
#[doc = "RX_CRC_ERROR_PACKETS (r) register accessor: Rx CRC error packets register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_error_packets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_error_packets`] module"]
#[doc(alias = "RX_CRC_ERROR_PACKETS")]
pub type RxCrcErrorPackets = crate::Reg<rx_crc_error_packets::RxCrcErrorPacketsSpec>;
#[doc = "Rx CRC error packets register"]
pub mod rx_crc_error_packets;
#[doc = "RX_ALIGNMENT_ERROR_PACKETS (r) register accessor: Rx alignment error packets register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_alignment_error_packets::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_alignment_error_packets`] module"]
#[doc(alias = "RX_ALIGNMENT_ERROR_PACKETS")]
pub type RxAlignmentErrorPackets =
    crate::Reg<rx_alignment_error_packets::RxAlignmentErrorPacketsSpec>;
#[doc = "Rx alignment error packets register"]
pub mod rx_alignment_error_packets;
#[doc = "RX_UNICAST_PACKETS_GOOD (r) register accessor: Rx unicast packets good register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_unicast_packets_good::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_unicast_packets_good`] module"]
#[doc(alias = "RX_UNICAST_PACKETS_GOOD")]
pub type RxUnicastPacketsGood = crate::Reg<rx_unicast_packets_good::RxUnicastPacketsGoodSpec>;
#[doc = "Rx unicast packets good register"]
pub mod rx_unicast_packets_good;
#[doc = "TX_LPI_USEC_CNTR (r) register accessor: Tx LPI microsecond timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_lpi_usec_cntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_lpi_usec_cntr`] module"]
#[doc(alias = "TX_LPI_USEC_CNTR")]
pub type TxLpiUsecCntr = crate::Reg<tx_lpi_usec_cntr::TxLpiUsecCntrSpec>;
#[doc = "Tx LPI microsecond timer register"]
pub mod tx_lpi_usec_cntr;
#[doc = "TX_LPI_TRAN_CNTR (r) register accessor: Tx LPI transition counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_lpi_tran_cntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_lpi_tran_cntr`] module"]
#[doc(alias = "TX_LPI_TRAN_CNTR")]
pub type TxLpiTranCntr = crate::Reg<tx_lpi_tran_cntr::TxLpiTranCntrSpec>;
#[doc = "Tx LPI transition counter register"]
pub mod tx_lpi_tran_cntr;
#[doc = "RX_LPI_USEC_CNTR (r) register accessor: Rx LPI microsecond counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_lpi_usec_cntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_lpi_usec_cntr`] module"]
#[doc(alias = "RX_LPI_USEC_CNTR")]
pub type RxLpiUsecCntr = crate::Reg<rx_lpi_usec_cntr::RxLpiUsecCntrSpec>;
#[doc = "Rx LPI microsecond counter register"]
pub mod rx_lpi_usec_cntr;
#[doc = "RX_LPI_TRAN_CNTR (r) register accessor: Rx LPI transition counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_lpi_tran_cntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_lpi_tran_cntr`] module"]
#[doc(alias = "RX_LPI_TRAN_CNTR")]
pub type RxLpiTranCntr = crate::Reg<rx_lpi_tran_cntr::RxLpiTranCntrSpec>;
#[doc = "Rx LPI transition counter register"]
pub mod rx_lpi_tran_cntr;
#[doc = "MACL3L4C0R (rw) register accessor: L3 and L4 control 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl3l4c0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3l4c0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macl3l4c0r`] module"]
#[doc(alias = "MACL3L4C0R")]
pub type Macl3l4c0r = crate::Reg<macl3l4c0r::Macl3l4c0rSpec>;
#[doc = "L3 and L4 control 0 register"]
pub mod macl3l4c0r;
#[doc = "MACL4A0R (rw) register accessor: Layer4 address filter 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl4a0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl4a0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macl4a0r`] module"]
#[doc(alias = "MACL4A0R")]
pub type Macl4a0r = crate::Reg<macl4a0r::Macl4a0rSpec>;
#[doc = "Layer4 address filter 0 register"]
pub mod macl4a0r;
#[doc = "MACDR (r) register accessor: Debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`macdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macdr`] module"]
#[doc(alias = "MACDR")]
pub type Macdr = crate::Reg<macdr::MacdrSpec>;
#[doc = "Debug register"]
pub mod macdr;
#[doc = "MACL3A00R (rw) register accessor: MACL3A00R\n\nYou can [`read`](crate::Reg::read) this register and get [`macl3a00r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a00r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macl3a00r`] module"]
#[doc(alias = "MACL3A00R")]
pub type Macl3a00r = crate::Reg<macl3a00r::Macl3a00rSpec>;
#[doc = "MACL3A00R"]
pub mod macl3a00r;
#[doc = "MACL3A10R (rw) register accessor: Layer3 address 1 filter 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl3a10r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a10r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macl3a10r`] module"]
#[doc(alias = "MACL3A10R")]
pub type Macl3a10r = crate::Reg<macl3a10r::Macl3a10rSpec>;
#[doc = "Layer3 address 1 filter 0 register"]
pub mod macl3a10r;
#[doc = "MACL3A20 (rw) register accessor: Layer3 Address 2 filter 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl3a20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macl3a20`] module"]
#[doc(alias = "MACL3A20")]
pub type Macl3a20 = crate::Reg<macl3a20::Macl3a20Spec>;
#[doc = "Layer3 Address 2 filter 0 register"]
pub mod macl3a20;
#[doc = "MACL3A30 (rw) register accessor: Layer3 Address 3 filter 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl3a30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macl3a30`] module"]
#[doc(alias = "MACL3A30")]
pub type Macl3a30 = crate::Reg<macl3a30::Macl3a30Spec>;
#[doc = "Layer3 Address 3 filter 0 register"]
pub mod macl3a30;
#[doc = "MACL3L4C1R (rw) register accessor: L3 and L4 control 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl3l4c1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3l4c1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macl3l4c1r`] module"]
#[doc(alias = "MACL3L4C1R")]
pub type Macl3l4c1r = crate::Reg<macl3l4c1r::Macl3l4c1rSpec>;
#[doc = "L3 and L4 control 1 register"]
pub mod macl3l4c1r;
#[doc = "MACL4A1R (rw) register accessor: Layer 4 address filter 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl4a1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl4a1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macl4a1r`] module"]
#[doc(alias = "MACL4A1R")]
pub type Macl4a1r = crate::Reg<macl4a1r::Macl4a1rSpec>;
#[doc = "Layer 4 address filter 1 register"]
pub mod macl4a1r;
#[doc = "MACL3A01R (rw) register accessor: Layer3 address 0 filter 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl3a01r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a01r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macl3a01r`] module"]
#[doc(alias = "MACL3A01R")]
pub type Macl3a01r = crate::Reg<macl3a01r::Macl3a01rSpec>;
#[doc = "Layer3 address 0 filter 1 Register"]
pub mod macl3a01r;
#[doc = "MACL3A11R (rw) register accessor: Layer3 address 1 filter 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl3a11r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a11r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macl3a11r`] module"]
#[doc(alias = "MACL3A11R")]
pub type Macl3a11r = crate::Reg<macl3a11r::Macl3a11rSpec>;
#[doc = "Layer3 address 1 filter 1 register"]
pub mod macl3a11r;
#[doc = "MACL3A21R (rw) register accessor: Layer3 address 2 filter 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl3a21r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a21r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macl3a21r`] module"]
#[doc(alias = "MACL3A21R")]
pub type Macl3a21r = crate::Reg<macl3a21r::Macl3a21rSpec>;
#[doc = "Layer3 address 2 filter 1 Register"]
pub mod macl3a21r;
#[doc = "MACL3A31R (rw) register accessor: Layer3 address 3 filter 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl3a31r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a31r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macl3a31r`] module"]
#[doc(alias = "MACL3A31R")]
pub type Macl3a31r = crate::Reg<macl3a31r::Macl3a31rSpec>;
#[doc = "Layer3 address 3 filter 1 register"]
pub mod macl3a31r;
#[doc = "MACTSCR (rw) register accessor: Timestamp control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mactscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mactscr`] module"]
#[doc(alias = "MACTSCR")]
pub type Mactscr = crate::Reg<mactscr::MactscrSpec>;
#[doc = "Timestamp control Register"]
pub mod mactscr;
#[doc = "MACSSIR (rw) register accessor: Sub-second increment register\n\nYou can [`read`](crate::Reg::read) this register and get [`macssir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macssir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macssir`] module"]
#[doc(alias = "MACSSIR")]
pub type Macssir = crate::Reg<macssir::MacssirSpec>;
#[doc = "Sub-second increment register"]
pub mod macssir;
#[doc = "MACSTSR (r) register accessor: System time seconds register\n\nYou can [`read`](crate::Reg::read) this register and get [`macstsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macstsr`] module"]
#[doc(alias = "MACSTSR")]
pub type Macstsr = crate::Reg<macstsr::MacstsrSpec>;
#[doc = "System time seconds register"]
pub mod macstsr;
#[doc = "MACSTNR (r) register accessor: System time nanoseconds register\n\nYou can [`read`](crate::Reg::read) this register and get [`macstnr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macstnr`] module"]
#[doc(alias = "MACSTNR")]
pub type Macstnr = crate::Reg<macstnr::MacstnrSpec>;
#[doc = "System time nanoseconds register"]
pub mod macstnr;
#[doc = "MACSTSUR (rw) register accessor: System time seconds update register\n\nYou can [`read`](crate::Reg::read) this register and get [`macstsur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macstsur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macstsur`] module"]
#[doc(alias = "MACSTSUR")]
pub type Macstsur = crate::Reg<macstsur::MacstsurSpec>;
#[doc = "System time seconds update register"]
pub mod macstsur;
#[doc = "MACSTNUR (rw) register accessor: System time nanoseconds update register\n\nYou can [`read`](crate::Reg::read) this register and get [`macstnur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macstnur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macstnur`] module"]
#[doc(alias = "MACSTNUR")]
pub type Macstnur = crate::Reg<macstnur::MacstnurSpec>;
#[doc = "System time nanoseconds update register"]
pub mod macstnur;
#[doc = "MACTSAR (rw) register accessor: Timestamp addend register\n\nYou can [`read`](crate::Reg::read) this register and get [`mactsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mactsar`] module"]
#[doc(alias = "MACTSAR")]
pub type Mactsar = crate::Reg<mactsar::MactsarSpec>;
#[doc = "Timestamp addend register"]
pub mod mactsar;
#[doc = "MACTSSR (r) register accessor: Timestamp status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mactssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mactssr`] module"]
#[doc(alias = "MACTSSR")]
pub type Mactssr = crate::Reg<mactssr::MactssrSpec>;
#[doc = "Timestamp status register"]
pub mod mactssr;
#[doc = "MACTxTSSNR (r) register accessor: Tx timestamp status nanoseconds register\n\nYou can [`read`](crate::Reg::read) this register and get [`mactx_tssnr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mactx_tssnr`] module"]
#[doc(alias = "MACTxTSSNR")]
pub type MactxTssnr = crate::Reg<mactx_tssnr::MactxTssnrSpec>;
#[doc = "Tx timestamp status nanoseconds register"]
pub mod mactx_tssnr;
#[doc = "MACTxTSSSR (r) register accessor: Tx timestamp status seconds register\n\nYou can [`read`](crate::Reg::read) this register and get [`mactx_tsssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mactx_tsssr`] module"]
#[doc(alias = "MACTxTSSSR")]
pub type MactxTsssr = crate::Reg<mactx_tsssr::MactxTsssrSpec>;
#[doc = "Tx timestamp status seconds register"]
pub mod mactx_tsssr;
#[doc = "MACACR (rw) register accessor: Auxiliary control register\n\nYou can [`read`](crate::Reg::read) this register and get [`macacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macacr`] module"]
#[doc(alias = "MACACR")]
pub type Macacr = crate::Reg<macacr::MacacrSpec>;
#[doc = "Auxiliary control register"]
pub mod macacr;
#[doc = "MACATSNR (r) register accessor: Auxiliary timestamp nanoseconds register\n\nYou can [`read`](crate::Reg::read) this register and get [`macatsnr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macatsnr`] module"]
#[doc(alias = "MACATSNR")]
pub type Macatsnr = crate::Reg<macatsnr::MacatsnrSpec>;
#[doc = "Auxiliary timestamp nanoseconds register"]
pub mod macatsnr;
#[doc = "MACATSSR (r) register accessor: Auxiliary timestamp seconds register\n\nYou can [`read`](crate::Reg::read) this register and get [`macatssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macatssr`] module"]
#[doc(alias = "MACATSSR")]
pub type Macatssr = crate::Reg<macatssr::MacatssrSpec>;
#[doc = "Auxiliary timestamp seconds register"]
pub mod macatssr;
#[doc = "MACTSIACR (rw) register accessor: Timestamp Ingress asymmetric correction register\n\nYou can [`read`](crate::Reg::read) this register and get [`mactsiacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsiacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mactsiacr`] module"]
#[doc(alias = "MACTSIACR")]
pub type Mactsiacr = crate::Reg<mactsiacr::MactsiacrSpec>;
#[doc = "Timestamp Ingress asymmetric correction register"]
pub mod mactsiacr;
#[doc = "MACTSEACR (rw) register accessor: Timestamp Egress asymmetric correction register\n\nYou can [`read`](crate::Reg::read) this register and get [`mactseacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactseacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mactseacr`] module"]
#[doc(alias = "MACTSEACR")]
pub type Mactseacr = crate::Reg<mactseacr::MactseacrSpec>;
#[doc = "Timestamp Egress asymmetric correction register"]
pub mod mactseacr;
#[doc = "MACTSICNR (rw) register accessor: Timestamp Ingress correction nanosecond register\n\nYou can [`read`](crate::Reg::read) this register and get [`mactsicnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsicnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mactsicnr`] module"]
#[doc(alias = "MACTSICNR")]
pub type Mactsicnr = crate::Reg<mactsicnr::MactsicnrSpec>;
#[doc = "Timestamp Ingress correction nanosecond register"]
pub mod mactsicnr;
#[doc = "MACTSECNR (rw) register accessor: Timestamp Egress correction nanosecond register\n\nYou can [`read`](crate::Reg::read) this register and get [`mactsecnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsecnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mactsecnr`] module"]
#[doc(alias = "MACTSECNR")]
pub type Mactsecnr = crate::Reg<mactsecnr::MactsecnrSpec>;
#[doc = "Timestamp Egress correction nanosecond register"]
pub mod mactsecnr;
#[doc = "MACPPSCR (rw) register accessor: PPS control register\n\nYou can [`read`](crate::Reg::read) this register and get [`macppscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macppscr`] module"]
#[doc(alias = "MACPPSCR")]
pub type Macppscr = crate::Reg<macppscr::MacppscrSpec>;
#[doc = "PPS control register"]
pub mod macppscr;
#[doc = "MACPPSTTSR (rw) register accessor: PPS target time seconds register\n\nYou can [`read`](crate::Reg::read) this register and get [`macppsttsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsttsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macppsttsr`] module"]
#[doc(alias = "MACPPSTTSR")]
pub type Macppsttsr = crate::Reg<macppsttsr::MacppsttsrSpec>;
#[doc = "PPS target time seconds register"]
pub mod macppsttsr;
#[doc = "MACPPSTTNR (rw) register accessor: PPS target time nanoseconds register\n\nYou can [`read`](crate::Reg::read) this register and get [`macppsttnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsttnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macppsttnr`] module"]
#[doc(alias = "MACPPSTTNR")]
pub type Macppsttnr = crate::Reg<macppsttnr::MacppsttnrSpec>;
#[doc = "PPS target time nanoseconds register"]
pub mod macppsttnr;
#[doc = "MACPPSIR (rw) register accessor: PPS interval register\n\nYou can [`read`](crate::Reg::read) this register and get [`macppsir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macppsir`] module"]
#[doc(alias = "MACPPSIR")]
pub type Macppsir = crate::Reg<macppsir::MacppsirSpec>;
#[doc = "PPS interval register"]
pub mod macppsir;
#[doc = "MACPPSWR (rw) register accessor: PPS width register\n\nYou can [`read`](crate::Reg::read) this register and get [`macppswr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppswr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macppswr`] module"]
#[doc(alias = "MACPPSWR")]
pub type Macppswr = crate::Reg<macppswr::MacppswrSpec>;
#[doc = "PPS width register"]
pub mod macppswr;
#[doc = "MACPOCR (rw) register accessor: PTP Offload control register\n\nYou can [`read`](crate::Reg::read) this register and get [`macpocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macpocr`] module"]
#[doc(alias = "MACPOCR")]
pub type Macpocr = crate::Reg<macpocr::MacpocrSpec>;
#[doc = "PTP Offload control register"]
pub mod macpocr;
#[doc = "MACSPI0R (rw) register accessor: PTP Source Port Identity 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`macspi0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macspi0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macspi0r`] module"]
#[doc(alias = "MACSPI0R")]
pub type Macspi0r = crate::Reg<macspi0r::Macspi0rSpec>;
#[doc = "PTP Source Port Identity 0 Register"]
pub mod macspi0r;
#[doc = "MACSPI1R (rw) register accessor: PTP Source port identity 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macspi1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macspi1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macspi1r`] module"]
#[doc(alias = "MACSPI1R")]
pub type Macspi1r = crate::Reg<macspi1r::Macspi1rSpec>;
#[doc = "PTP Source port identity 1 register"]
pub mod macspi1r;
#[doc = "MACSPI2R (rw) register accessor: PTP Source port identity 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macspi2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macspi2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macspi2r`] module"]
#[doc(alias = "MACSPI2R")]
pub type Macspi2r = crate::Reg<macspi2r::Macspi2rSpec>;
#[doc = "PTP Source port identity 2 register"]
pub mod macspi2r;
#[doc = "MACLMIR (rw) register accessor: Log message interval register\n\nYou can [`read`](crate::Reg::read) this register and get [`maclmir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maclmir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maclmir`] module"]
#[doc(alias = "MACLMIR")]
pub type Maclmir = crate::Reg<maclmir::MaclmirSpec>;
#[doc = "Log message interval register"]
pub mod maclmir;
