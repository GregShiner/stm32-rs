#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `RXP` reader - Rx-Packet available"]
pub type RxpR = crate::BitReader;
#[doc = "Field `TXP` reader - Tx-Packet space available"]
pub type TxpR = crate::BitReader;
#[doc = "Field `DXP` reader - Duplex Packet"]
pub type DxpR = crate::BitReader;
#[doc = "Field `EOT` reader - End Of Transfer"]
pub type EotR = crate::BitReader;
#[doc = "Field `TXTF` reader - Transmission Transfer Filled"]
pub type TxtfR = crate::BitReader;
#[doc = "Field `UDR` reader - Underrun at slave transmission mode"]
pub type UdrR = crate::BitReader;
#[doc = "Field `OVR` reader - Overrun"]
pub type OvrR = crate::BitReader;
#[doc = "Field `CRCE` reader - CRC Error"]
pub type CrceR = crate::BitReader;
#[doc = "Field `TIFRE` reader - TI frame format error"]
pub type TifreR = crate::BitReader;
#[doc = "Field `MODF` reader - Mode Fault"]
pub type ModfR = crate::BitReader;
#[doc = "Field `TSERF` reader - Additional number of SPI data to be transacted was reload"]
pub type TserfR = crate::BitReader;
#[doc = "Field `SUSP` reader - SUSPend"]
pub type SuspR = crate::BitReader;
#[doc = "Field `TXC` reader - TxFIFO transmission complete"]
pub type TxcR = crate::BitReader;
#[doc = "Field `RXPLVL` reader - RxFIFO Packing LeVeL"]
pub type RxplvlR = crate::FieldReader;
#[doc = "Field `RXWNE` reader - RxFIFO Word Not Empty"]
pub type RxwneR = crate::BitReader;
#[doc = "Field `CTSIZE` reader - Number of data frames remaining in current TSIZE session"]
pub type CtsizeR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Rx-Packet available"]
    #[inline(always)]
    pub fn rxp(&self) -> RxpR {
        RxpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx-Packet space available"]
    #[inline(always)]
    pub fn txp(&self) -> TxpR {
        TxpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Duplex Packet"]
    #[inline(always)]
    pub fn dxp(&self) -> DxpR {
        DxpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End Of Transfer"]
    #[inline(always)]
    pub fn eot(&self) -> EotR {
        EotR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmission Transfer Filled"]
    #[inline(always)]
    pub fn txtf(&self) -> TxtfR {
        TxtfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Underrun at slave transmission mode"]
    #[inline(always)]
    pub fn udr(&self) -> UdrR {
        UdrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun"]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC Error"]
    #[inline(always)]
    pub fn crce(&self) -> CrceR {
        CrceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TI frame format error"]
    #[inline(always)]
    pub fn tifre(&self) -> TifreR {
        TifreR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mode Fault"]
    #[inline(always)]
    pub fn modf(&self) -> ModfR {
        ModfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Additional number of SPI data to be transacted was reload"]
    #[inline(always)]
    pub fn tserf(&self) -> TserfR {
        TserfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SUSPend"]
    #[inline(always)]
    pub fn susp(&self) -> SuspR {
        SuspR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TxFIFO transmission complete"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - RxFIFO Packing LeVeL"]
    #[inline(always)]
    pub fn rxplvl(&self) -> RxplvlR {
        RxplvlR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - RxFIFO Word Not Empty"]
    #[inline(always)]
    pub fn rxwne(&self) -> RxwneR {
        RxwneR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Number of data frames remaining in current TSIZE session"]
    #[inline(always)]
    pub fn ctsize(&self) -> CtsizeR {
        CtsizeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0x1002"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x1002;
}
