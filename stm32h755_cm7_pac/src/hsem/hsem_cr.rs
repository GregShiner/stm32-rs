#[doc = "Register `HSEM_CR` writer"]
pub type W = crate::W<HsemCrSpec>;
#[doc = "Field `COREID` writer - COREID of semaphores to be cleared"]
pub type CoreidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `KEY` writer - Semaphore clear Key"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 8:11 - COREID of semaphores to be cleared"]
    #[inline(always)]
    pub fn coreid(&mut self) -> CoreidW<HsemCrSpec> {
        CoreidW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Semaphore clear Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<HsemCrSpec> {
        KeyW::new(self, 16)
    }
}
#[doc = "HSEM Clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsemCrSpec;
impl crate::RegisterSpec for HsemCrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hsem_cr::W`](W) writer structure"]
impl crate::Writable for HsemCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSEM_CR to value 0"]
impl crate::Resettable for HsemCrSpec {}
