#[doc = "Register `S5PAR` reader"]
pub type R = crate::R<S5parSpec>;
#[doc = "Register `S5PAR` writer"]
pub type W = crate::W<S5parSpec>;
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
    pub fn pa(&mut self) -> PaW<S5parSpec> {
        PaW::new(self, 0)
    }
}
#[doc = "stream x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s5par::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s5par::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S5parSpec;
impl crate::RegisterSpec for S5parSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s5par::R`](R) reader structure"]
impl crate::Readable for S5parSpec {}
#[doc = "`write(|w| ..)` method takes [`s5par::W`](W) writer structure"]
impl crate::Writable for S5parSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S5PAR to value 0"]
impl crate::Resettable for S5parSpec {}
