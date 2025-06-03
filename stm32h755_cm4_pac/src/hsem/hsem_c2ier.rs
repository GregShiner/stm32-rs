#[doc = "Register `HSEM_C2IER` reader"]
pub type R = crate::R<HsemC2ierSpec>;
#[doc = "Register `HSEM_C2IER` writer"]
pub type W = crate::W<HsemC2ierSpec>;
#[doc = "Field `ISE` reader - Interrupt semaphore x enable bit"]
pub type IseR = crate::FieldReader<u32>;
#[doc = "Field `ISE` writer - Interrupt semaphore x enable bit"]
pub type IseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt semaphore x enable bit"]
    #[inline(always)]
    pub fn ise(&self) -> IseR {
        IseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt semaphore x enable bit"]
    #[inline(always)]
    pub fn ise(&mut self) -> IseW<HsemC2ierSpec> {
        IseW::new(self, 0)
    }
}
#[doc = "HSEM Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_c2ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_c2ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsemC2ierSpec;
impl crate::RegisterSpec for HsemC2ierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_c2ier::R`](R) reader structure"]
impl crate::Readable for HsemC2ierSpec {}
#[doc = "`write(|w| ..)` method takes [`hsem_c2ier::W`](W) writer structure"]
impl crate::Writable for HsemC2ierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSEM_C2IER to value 0"]
impl crate::Resettable for HsemC2ierSpec {}
