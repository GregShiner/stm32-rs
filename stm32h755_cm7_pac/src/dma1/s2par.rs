#[doc = "Register `S2PAR` reader"]
pub type R = crate::R<S2parSpec>;
#[doc = "Register `S2PAR` writer"]
pub type W = crate::W<S2parSpec>;
#[doc = "Field `PA` reader - Peripheral address"]
pub type PaR = crate::FieldReader<u32>;
#[doc = "Field `PA` writer - Peripheral address"]
pub type PaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&self) -> PaR {
        PaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PaW<S2parSpec> {
        PaW::new(self, 0)
    }
}
#[doc = "stream x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s2par::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2par::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2parSpec;
impl crate::RegisterSpec for S2parSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s2par::R`](R) reader structure"]
impl crate::Readable for S2parSpec {}
#[doc = "`write(|w| ..)` method takes [`s2par::W`](W) writer structure"]
impl crate::Writable for S2parSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S2PAR to value 0"]
impl crate::Resettable for S2parSpec {}
