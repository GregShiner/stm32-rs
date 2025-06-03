#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `RXBFF` reader - Receive buffer full flag"]
pub type RxbffR = crate::BitReader;
#[doc = "Field `TXBEF` reader - Transmit buffer empty flag"]
pub type TxbefR = crate::BitReader;
#[doc = "Field `RXBERF` reader - Receive CRC error flag"]
pub type RxberfR = crate::BitReader;
#[doc = "Field `RXOVRF` reader - Receive overrun error flag"]
pub type RxovrfR = crate::BitReader;
#[doc = "Field `TXUNRF` reader - Transmit underrun error flag"]
pub type TxunrfR = crate::BitReader;
#[doc = "Field `RXNE` reader - Receive data register not empty"]
pub type RxneR = crate::BitReader;
#[doc = "Field `TXE` reader - Transmit data register empty"]
pub type TxeR = crate::BitReader;
#[doc = "Field `TCF` reader - Transfer complete flag"]
pub type TcfR = crate::BitReader;
#[doc = "Field `SRF` reader - Slave resume flag"]
pub type SrfR = crate::BitReader;
#[doc = "Field `SUSP` reader - SUSPEND flag"]
pub type SuspR = crate::BitReader;
#[doc = "Field `DEACTF` reader - DEACTIVATED flag"]
pub type DeactfR = crate::BitReader;
#[doc = "Field `RDYF` reader - transceiver ready flag"]
pub type RdyfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receive buffer full flag"]
    #[inline(always)]
    pub fn rxbff(&self) -> RxbffR {
        RxbffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty flag"]
    #[inline(always)]
    pub fn txbef(&self) -> TxbefR {
        TxbefR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive CRC error flag"]
    #[inline(always)]
    pub fn rxberf(&self) -> RxberfR {
        RxberfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive overrun error flag"]
    #[inline(always)]
    pub fn rxovrf(&self) -> RxovrfR {
        RxovrfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit underrun error flag"]
    #[inline(always)]
    pub fn txunrf(&self) -> TxunrfR {
        TxunrfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive data register not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit data register empty"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transfer complete flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TcfR {
        TcfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Slave resume flag"]
    #[inline(always)]
    pub fn srf(&self) -> SrfR {
        SrfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SUSPEND flag"]
    #[inline(always)]
    pub fn susp(&self) -> SuspR {
        SuspR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DEACTIVATED flag"]
    #[inline(always)]
    pub fn deactf(&self) -> DeactfR {
        DeactfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - transceiver ready flag"]
    #[inline(always)]
    pub fn rdyf(&self) -> RdyfR {
        RdyfR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "SWPMI Interrupt and Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0x02c2"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0x02c2;
}
