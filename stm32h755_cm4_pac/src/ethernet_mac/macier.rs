#[doc = "Register `MACIER` reader"]
pub type R = crate::R<MacierSpec>;
#[doc = "Register `MACIER` writer"]
pub type W = crate::W<MacierSpec>;
#[doc = "Field `PHYIE` reader - PHYIE"]
pub type PhyieR = crate::BitReader;
#[doc = "Field `PHYIE` writer - PHYIE"]
pub type PhyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMTIE` reader - PMTIE"]
pub type PmtieR = crate::BitReader;
#[doc = "Field `PMTIE` writer - PMTIE"]
pub type PmtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPIIE` reader - LPIIE"]
pub type LpiieR = crate::BitReader;
#[doc = "Field `LPIIE` writer - LPIIE"]
pub type LpiieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIE` reader - TSIE"]
pub type TsieR = crate::BitReader;
#[doc = "Field `TSIE` writer - TSIE"]
pub type TsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSTSIE` reader - TXSTSIE"]
pub type TxstsieR = crate::BitReader;
#[doc = "Field `TXSTSIE` writer - TXSTSIE"]
pub type TxstsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTSIE` reader - RXSTSIE"]
pub type RxstsieR = crate::BitReader;
#[doc = "Field `RXSTSIE` writer - RXSTSIE"]
pub type RxstsieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - PHYIE"]
    #[inline(always)]
    pub fn phyie(&self) -> PhyieR {
        PhyieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMTIE"]
    #[inline(always)]
    pub fn pmtie(&self) -> PmtieR {
        PmtieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPIIE"]
    #[inline(always)]
    pub fn lpiie(&self) -> LpiieR {
        LpiieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - TSIE"]
    #[inline(always)]
    pub fn tsie(&self) -> TsieR {
        TsieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TXSTSIE"]
    #[inline(always)]
    pub fn txstsie(&self) -> TxstsieR {
        TxstsieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RXSTSIE"]
    #[inline(always)]
    pub fn rxstsie(&self) -> RxstsieR {
        RxstsieR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PHYIE"]
    #[inline(always)]
    pub fn phyie(&mut self) -> PhyieW<MacierSpec> {
        PhyieW::new(self, 3)
    }
    #[doc = "Bit 4 - PMTIE"]
    #[inline(always)]
    pub fn pmtie(&mut self) -> PmtieW<MacierSpec> {
        PmtieW::new(self, 4)
    }
    #[doc = "Bit 5 - LPIIE"]
    #[inline(always)]
    pub fn lpiie(&mut self) -> LpiieW<MacierSpec> {
        LpiieW::new(self, 5)
    }
    #[doc = "Bit 12 - TSIE"]
    #[inline(always)]
    pub fn tsie(&mut self) -> TsieW<MacierSpec> {
        TsieW::new(self, 12)
    }
    #[doc = "Bit 13 - TXSTSIE"]
    #[inline(always)]
    pub fn txstsie(&mut self) -> TxstsieW<MacierSpec> {
        TxstsieW::new(self, 13)
    }
    #[doc = "Bit 14 - RXSTSIE"]
    #[inline(always)]
    pub fn rxstsie(&mut self) -> RxstsieW<MacierSpec> {
        RxstsieW::new(self, 14)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`macier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacierSpec;
impl crate::RegisterSpec for MacierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macier::R`](R) reader structure"]
impl crate::Readable for MacierSpec {}
#[doc = "`write(|w| ..)` method takes [`macier::W`](W) writer structure"]
impl crate::Writable for MacierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACIER to value 0"]
impl crate::Resettable for MacierSpec {}
