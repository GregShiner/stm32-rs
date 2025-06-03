#[doc = "Register `HSEM_C1ICR` reader"]
pub type R = crate::R<HsemC1icrSpec>;
#[doc = "Register `HSEM_C1ICR` writer"]
pub type W = crate::W<HsemC1icrSpec>;
#[doc = "Field `ISC` reader - Interrupt semaphore x clear bit"]
pub type IscR = crate::FieldReader<u32>;
#[doc = "Field `ISC` writer - Interrupt semaphore x clear bit"]
pub type IscW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt semaphore x clear bit"]
    #[inline(always)]
    pub fn isc(&self) -> IscR {
        IscR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt semaphore x clear bit"]
    #[inline(always)]
    pub fn isc(&mut self) -> IscW<HsemC1icrSpec> {
        IscW::new(self, 0)
    }
}
#[doc = "HSEM Interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_c1icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_c1icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsemC1icrSpec;
impl crate::RegisterSpec for HsemC1icrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_c1icr::R`](R) reader structure"]
impl crate::Readable for HsemC1icrSpec {}
#[doc = "`write(|w| ..)` method takes [`hsem_c1icr::W`](W) writer structure"]
impl crate::Writable for HsemC1icrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSEM_C1ICR to value 0"]
impl crate::Resettable for HsemC1icrSpec {}
