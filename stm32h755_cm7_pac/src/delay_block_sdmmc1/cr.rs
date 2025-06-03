#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `DEN` reader - Delay block enable bit"]
pub type DenR = crate::BitReader;
#[doc = "Field `DEN` writer - Delay block enable bit"]
pub type DenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEN` reader - Sampler length enable bit"]
pub type SenR = crate::BitReader;
#[doc = "Field `SEN` writer - Sampler length enable bit"]
pub type SenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Delay block enable bit"]
    #[inline(always)]
    pub fn den(&self) -> DenR {
        DenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sampler length enable bit"]
    #[inline(always)]
    pub fn sen(&self) -> SenR {
        SenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Delay block enable bit"]
    #[inline(always)]
    pub fn den(&mut self) -> DenW<CrSpec> {
        DenW::new(self, 0)
    }
    #[doc = "Bit 1 - Sampler length enable bit"]
    #[inline(always)]
    pub fn sen(&mut self) -> SenW<CrSpec> {
        SenW::new(self, 1)
    }
}
#[doc = "DLYB control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
