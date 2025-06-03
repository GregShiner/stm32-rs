#[doc = "Register `M5CR` reader"]
pub type R = crate::R<M5crSpec>;
#[doc = "Register `M5CR` writer"]
pub type W = crate::W<M5crSpec>;
#[doc = "Field `ECCSEIE` reader - ECC single error interrupt enable"]
pub type EccseieR = crate::BitReader;
#[doc = "Field `ECCSEIE` writer - ECC single error interrupt enable"]
pub type EccseieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCDEIE` reader - ECC double error interrupt enable"]
pub type EccdeieR = crate::BitReader;
#[doc = "Field `ECCDEIE` writer - ECC double error interrupt enable"]
pub type EccdeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCDEBWIE` reader - ECC double error on byte write (BW) interrupt enable"]
pub type EccdebwieR = crate::BitReader;
#[doc = "Field `ECCDEBWIE` writer - ECC double error on byte write (BW) interrupt enable"]
pub type EccdebwieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCELEN` reader - ECC error latching enable"]
pub type EccelenR = crate::BitReader;
#[doc = "Field `ECCELEN` writer - ECC error latching enable"]
pub type EccelenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - ECC single error interrupt enable"]
    #[inline(always)]
    pub fn eccseie(&self) -> EccseieR {
        EccseieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC double error interrupt enable"]
    #[inline(always)]
    pub fn eccdeie(&self) -> EccdeieR {
        EccdeieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ECC double error on byte write (BW) interrupt enable"]
    #[inline(always)]
    pub fn eccdebwie(&self) -> EccdebwieR {
        EccdebwieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ECC error latching enable"]
    #[inline(always)]
    pub fn eccelen(&self) -> EccelenR {
        EccelenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - ECC single error interrupt enable"]
    #[inline(always)]
    pub fn eccseie(&mut self) -> EccseieW<M5crSpec> {
        EccseieW::new(self, 2)
    }
    #[doc = "Bit 3 - ECC double error interrupt enable"]
    #[inline(always)]
    pub fn eccdeie(&mut self) -> EccdeieW<M5crSpec> {
        EccdeieW::new(self, 3)
    }
    #[doc = "Bit 4 - ECC double error on byte write (BW) interrupt enable"]
    #[inline(always)]
    pub fn eccdebwie(&mut self) -> EccdebwieW<M5crSpec> {
        EccdebwieW::new(self, 4)
    }
    #[doc = "Bit 5 - ECC error latching enable"]
    #[inline(always)]
    pub fn eccelen(&mut self) -> EccelenW<M5crSpec> {
        EccelenW::new(self, 5)
    }
}
#[doc = "RAMECC monitor x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`m5cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M5crSpec;
impl crate::RegisterSpec for M5crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m5cr::R`](R) reader structure"]
impl crate::Readable for M5crSpec {}
#[doc = "`write(|w| ..)` method takes [`m5cr::W`](W) writer structure"]
impl crate::Writable for M5crSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets M5CR to value 0"]
impl crate::Resettable for M5crSpec {}
