#[doc = "Register `OTG_HS_GRSTCTL` reader"]
pub type R = crate::R<OtgHsGrstctlSpec>;
#[doc = "Register `OTG_HS_GRSTCTL` writer"]
pub type W = crate::W<OtgHsGrstctlSpec>;
#[doc = "Field `CSRST` reader - Core soft reset"]
pub type CsrstR = crate::BitReader;
#[doc = "Field `CSRST` writer - Core soft reset"]
pub type CsrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSRST` reader - HCLK soft reset"]
pub type HsrstR = crate::BitReader;
#[doc = "Field `HSRST` writer - HCLK soft reset"]
pub type HsrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCRST` reader - Host frame counter reset"]
pub type FcrstR = crate::BitReader;
#[doc = "Field `FCRST` writer - Host frame counter reset"]
pub type FcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFLSH` reader - RxFIFO flush"]
pub type RxfflshR = crate::BitReader;
#[doc = "Field `RXFFLSH` writer - RxFIFO flush"]
pub type RxfflshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFFLSH` reader - TxFIFO flush"]
pub type TxfflshR = crate::BitReader;
#[doc = "Field `TXFFLSH` writer - TxFIFO flush"]
pub type TxfflshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - TxFIFO number"]
pub type TxfnumR = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - TxFIFO number"]
pub type TxfnumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DMAREQ` reader - DMA request signal enabled for USB OTG HS"]
pub type DmareqR = crate::BitReader;
#[doc = "Field `AHBIDL` reader - AHB master idle"]
pub type AhbidlR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Core soft reset"]
    #[inline(always)]
    pub fn csrst(&self) -> CsrstR {
        CsrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HCLK soft reset"]
    #[inline(always)]
    pub fn hsrst(&self) -> HsrstR {
        HsrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host frame counter reset"]
    #[inline(always)]
    pub fn fcrst(&self) -> FcrstR {
        FcrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO flush"]
    #[inline(always)]
    pub fn rxfflsh(&self) -> RxfflshR {
        RxfflshR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TxFIFO flush"]
    #[inline(always)]
    pub fn txfflsh(&self) -> TxfflshR {
        TxfflshR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - TxFIFO number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TxfnumR {
        TxfnumR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - DMA request signal enabled for USB OTG HS"]
    #[inline(always)]
    pub fn dmareq(&self) -> DmareqR {
        DmareqR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AHB master idle"]
    #[inline(always)]
    pub fn ahbidl(&self) -> AhbidlR {
        AhbidlR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core soft reset"]
    #[inline(always)]
    pub fn csrst(&mut self) -> CsrstW<OtgHsGrstctlSpec> {
        CsrstW::new(self, 0)
    }
    #[doc = "Bit 1 - HCLK soft reset"]
    #[inline(always)]
    pub fn hsrst(&mut self) -> HsrstW<OtgHsGrstctlSpec> {
        HsrstW::new(self, 1)
    }
    #[doc = "Bit 2 - Host frame counter reset"]
    #[inline(always)]
    pub fn fcrst(&mut self) -> FcrstW<OtgHsGrstctlSpec> {
        FcrstW::new(self, 2)
    }
    #[doc = "Bit 4 - RxFIFO flush"]
    #[inline(always)]
    pub fn rxfflsh(&mut self) -> RxfflshW<OtgHsGrstctlSpec> {
        RxfflshW::new(self, 4)
    }
    #[doc = "Bit 5 - TxFIFO flush"]
    #[inline(always)]
    pub fn txfflsh(&mut self) -> TxfflshW<OtgHsGrstctlSpec> {
        TxfflshW::new(self, 5)
    }
    #[doc = "Bits 6:10 - TxFIFO number"]
    #[inline(always)]
    pub fn txfnum(&mut self) -> TxfnumW<OtgHsGrstctlSpec> {
        TxfnumW::new(self, 6)
    }
}
#[doc = "OTG_HS reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_grstctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_grstctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsGrstctlSpec;
impl crate::RegisterSpec for OtgHsGrstctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_grstctl::R`](R) reader structure"]
impl crate::Readable for OtgHsGrstctlSpec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_grstctl::W`](W) writer structure"]
impl crate::Writable for OtgHsGrstctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_GRSTCTL to value 0x2000_0000"]
impl crate::Resettable for OtgHsGrstctlSpec {
    const RESET_VALUE: u32 = 0x2000_0000;
}
