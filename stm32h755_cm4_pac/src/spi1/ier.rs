#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `RXPIE` reader - RXP Interrupt Enable"]
pub type RxpieR = crate::BitReader;
#[doc = "Field `RXPIE` writer - RXP Interrupt Enable"]
pub type RxpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPIE` reader - TXP interrupt enable"]
pub type TxpieR = crate::BitReader;
#[doc = "Field `DPXPIE` reader - DXP interrupt enabled"]
pub type DpxpieR = crate::BitReader;
#[doc = "Field `EOTIE` reader - EOT, SUSP and TXC interrupt enable"]
pub type EotieR = crate::BitReader;
#[doc = "Field `EOTIE` writer - EOT, SUSP and TXC interrupt enable"]
pub type EotieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTFIE` reader - TXTFIE interrupt enable"]
pub type TxtfieR = crate::BitReader;
#[doc = "Field `TXTFIE` writer - TXTFIE interrupt enable"]
pub type TxtfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDRIE` reader - UDR interrupt enable"]
pub type UdrieR = crate::BitReader;
#[doc = "Field `UDRIE` writer - UDR interrupt enable"]
pub type UdrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRIE` reader - OVR interrupt enable"]
pub type OvrieR = crate::BitReader;
#[doc = "Field `OVRIE` writer - OVR interrupt enable"]
pub type OvrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEIE` reader - CRC Interrupt enable"]
pub type CrceieR = crate::BitReader;
#[doc = "Field `CRCEIE` writer - CRC Interrupt enable"]
pub type CrceieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIFREIE` reader - TIFRE interrupt enable"]
pub type TifreieR = crate::BitReader;
#[doc = "Field `TIFREIE` writer - TIFRE interrupt enable"]
pub type TifreieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODFIE` reader - Mode Fault interrupt enable"]
pub type ModfieR = crate::BitReader;
#[doc = "Field `MODFIE` writer - Mode Fault interrupt enable"]
pub type ModfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSERFIE` reader - Additional number of transactions reload interrupt enable"]
pub type TserfieR = crate::BitReader;
#[doc = "Field `TSERFIE` writer - Additional number of transactions reload interrupt enable"]
pub type TserfieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXP Interrupt Enable"]
    #[inline(always)]
    pub fn rxpie(&self) -> RxpieR {
        RxpieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXP interrupt enable"]
    #[inline(always)]
    pub fn txpie(&self) -> TxpieR {
        TxpieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DXP interrupt enabled"]
    #[inline(always)]
    pub fn dpxpie(&self) -> DpxpieR {
        DpxpieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOT, SUSP and TXC interrupt enable"]
    #[inline(always)]
    pub fn eotie(&self) -> EotieR {
        EotieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXTFIE interrupt enable"]
    #[inline(always)]
    pub fn txtfie(&self) -> TxtfieR {
        TxtfieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UDR interrupt enable"]
    #[inline(always)]
    pub fn udrie(&self) -> UdrieR {
        UdrieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OVR interrupt enable"]
    #[inline(always)]
    pub fn ovrie(&self) -> OvrieR {
        OvrieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC Interrupt enable"]
    #[inline(always)]
    pub fn crceie(&self) -> CrceieR {
        CrceieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIFRE interrupt enable"]
    #[inline(always)]
    pub fn tifreie(&self) -> TifreieR {
        TifreieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mode Fault interrupt enable"]
    #[inline(always)]
    pub fn modfie(&self) -> ModfieR {
        ModfieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Additional number of transactions reload interrupt enable"]
    #[inline(always)]
    pub fn tserfie(&self) -> TserfieR {
        TserfieR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXP Interrupt Enable"]
    #[inline(always)]
    pub fn rxpie(&mut self) -> RxpieW<IerSpec> {
        RxpieW::new(self, 0)
    }
    #[doc = "Bit 3 - EOT, SUSP and TXC interrupt enable"]
    #[inline(always)]
    pub fn eotie(&mut self) -> EotieW<IerSpec> {
        EotieW::new(self, 3)
    }
    #[doc = "Bit 4 - TXTFIE interrupt enable"]
    #[inline(always)]
    pub fn txtfie(&mut self) -> TxtfieW<IerSpec> {
        TxtfieW::new(self, 4)
    }
    #[doc = "Bit 5 - UDR interrupt enable"]
    #[inline(always)]
    pub fn udrie(&mut self) -> UdrieW<IerSpec> {
        UdrieW::new(self, 5)
    }
    #[doc = "Bit 6 - OVR interrupt enable"]
    #[inline(always)]
    pub fn ovrie(&mut self) -> OvrieW<IerSpec> {
        OvrieW::new(self, 6)
    }
    #[doc = "Bit 7 - CRC Interrupt enable"]
    #[inline(always)]
    pub fn crceie(&mut self) -> CrceieW<IerSpec> {
        CrceieW::new(self, 7)
    }
    #[doc = "Bit 8 - TIFRE interrupt enable"]
    #[inline(always)]
    pub fn tifreie(&mut self) -> TifreieW<IerSpec> {
        TifreieW::new(self, 8)
    }
    #[doc = "Bit 9 - Mode Fault interrupt enable"]
    #[inline(always)]
    pub fn modfie(&mut self) -> ModfieW<IerSpec> {
        ModfieW::new(self, 9)
    }
    #[doc = "Bit 10 - Additional number of transactions reload interrupt enable"]
    #[inline(always)]
    pub fn tserfie(&mut self) -> TserfieW<IerSpec> {
        TserfieW::new(self, 10)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
