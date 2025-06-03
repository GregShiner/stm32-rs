#[doc = "Register `HSEM_C2ICR` reader"]
pub type R = crate::R<HsemC2icrSpec>;
#[doc = "Register `HSEM_C2ICR` writer"]
pub type W = crate::W<HsemC2icrSpec>;
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
    pub fn isc(&mut self) -> IscW<HsemC2icrSpec> {
        IscW::new(self, 0)
    }
}
#[doc = "HSEM Interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_c2icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_c2icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsemC2icrSpec;
impl crate::RegisterSpec for HsemC2icrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_c2icr::R`](R) reader structure"]
impl crate::Readable for HsemC2icrSpec {}
#[doc = "`write(|w| ..)` method takes [`hsem_c2icr::W`](W) writer structure"]
impl crate::Writable for HsemC2icrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSEM_C2ICR to value 0"]
impl crate::Resettable for HsemC2icrSpec {}
