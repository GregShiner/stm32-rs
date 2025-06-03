#[doc = "Register `HSEM_KEYR` reader"]
pub type R = crate::R<HsemKeyrSpec>;
#[doc = "Register `HSEM_KEYR` writer"]
pub type W = crate::W<HsemKeyrSpec>;
#[doc = "Field `KEY` reader - Semaphore Clear Key"]
pub type KeyR = crate::FieldReader<u16>;
#[doc = "Field `KEY` writer - Semaphore Clear Key"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - Semaphore Clear Key"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Semaphore Clear Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<HsemKeyrSpec> {
        KeyW::new(self, 16)
    }
}
#[doc = "HSEM Interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_keyr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_keyr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsemKeyrSpec;
impl crate::RegisterSpec for HsemKeyrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_keyr::R`](R) reader structure"]
impl crate::Readable for HsemKeyrSpec {}
#[doc = "`write(|w| ..)` method takes [`hsem_keyr::W`](W) writer structure"]
impl crate::Writable for HsemKeyrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSEM_KEYR to value 0"]
impl crate::Resettable for HsemKeyrSpec {}
