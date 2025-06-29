#[doc = "Register `S7PAR` reader"]
pub type R = crate::R<S7parSpec>;
#[doc = "Register `S7PAR` writer"]
pub type W = crate::W<S7parSpec>;
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
    pub fn pa(&mut self) -> PaW<S7parSpec> {
        PaW::new(self, 0)
    }
}
#[doc = "stream x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s7par::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s7par::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S7parSpec;
impl crate::RegisterSpec for S7parSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s7par::R`](R) reader structure"]
impl crate::Readable for S7parSpec {}
#[doc = "`write(|w| ..)` method takes [`s7par::W`](W) writer structure"]
impl crate::Writable for S7parSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S7PAR to value 0"]
impl crate::Resettable for S7parSpec {}
