#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `PE` reader - PE"]
pub type PeR = crate::BitReader;
#[doc = "Field `FE` reader - FE"]
pub type FeR = crate::BitReader;
#[doc = "Field `NE` reader - NE"]
pub type NeR = crate::BitReader;
#[doc = "Field `ORE` reader - ORE"]
pub type OreR = crate::BitReader;
#[doc = "Field `IDLE` reader - IDLE"]
pub type IdleR = crate::BitReader;
#[doc = "Field `RXNE` reader - RXNE"]
pub type RxneR = crate::BitReader;
#[doc = "Field `TC` reader - TC"]
pub type TcR = crate::BitReader;
#[doc = "Field `TXE` reader - TXE"]
pub type TxeR = crate::BitReader;
#[doc = "Field `CTSIF` reader - CTSIF"]
pub type CtsifR = crate::BitReader;
#[doc = "Field `CTS` reader - CTS"]
pub type CtsR = crate::BitReader;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BusyR = crate::BitReader;
#[doc = "Field `CMF` reader - CMF"]
pub type CmfR = crate::BitReader;
#[doc = "Field `SBKF` reader - SBKF"]
pub type SbkfR = crate::BitReader;
#[doc = "Field `RWU` reader - RWU"]
pub type RwuR = crate::BitReader;
#[doc = "Field `WUF` reader - WUF"]
pub type WufR = crate::BitReader;
#[doc = "Field `TEACK` reader - TEACK"]
pub type TeackR = crate::BitReader;
#[doc = "Field `REACK` reader - REACK"]
pub type ReackR = crate::BitReader;
#[doc = "Field `TXFE` reader - TXFIFO Empty"]
pub type TxfeR = crate::BitReader;
#[doc = "Field `RXFF` reader - RXFIFO Full"]
pub type RxffR = crate::BitReader;
#[doc = "Field `RXFT` reader - RXFIFO threshold flag"]
pub type RxftR = crate::BitReader;
#[doc = "Field `TXFT` reader - TXFIFO threshold flag"]
pub type TxftR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PE"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FE"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NE"]
    #[inline(always)]
    pub fn ne(&self) -> NeR {
        NeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ORE"]
    #[inline(always)]
    pub fn ore(&self) -> OreR {
        OreR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TC"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - CTSIF"]
    #[inline(always)]
    pub fn ctsif(&self) -> CtsifR {
        CtsifR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CMF"]
    #[inline(always)]
    pub fn cmf(&self) -> CmfR {
        CmfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SBKF"]
    #[inline(always)]
    pub fn sbkf(&self) -> SbkfR {
        SbkfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RWU"]
    #[inline(always)]
    pub fn rwu(&self) -> RwuR {
        RwuR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - WUF"]
    #[inline(always)]
    pub fn wuf(&self) -> WufR {
        WufR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TEACK"]
    #[inline(always)]
    pub fn teack(&self) -> TeackR {
        TeackR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - REACK"]
    #[inline(always)]
    pub fn reack(&self) -> ReackR {
        ReackR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TXFIFO Empty"]
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RXFIFO Full"]
    #[inline(always)]
    pub fn rxff(&self) -> RxffR {
        RxffR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - RXFIFO threshold flag"]
    #[inline(always)]
    pub fn rxft(&self) -> RxftR {
        RxftR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TXFIFO threshold flag"]
    #[inline(always)]
    pub fn txft(&self) -> TxftR {
        TxftR::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "Interrupt & status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0xc0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0xc0;
}
