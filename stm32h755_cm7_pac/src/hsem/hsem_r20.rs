#[doc = "Register `HSEM_R20` reader"]
pub type R = crate::R<HsemR20Spec>;
#[doc = "Register `HSEM_R20` writer"]
pub type W = crate::W<HsemR20Spec>;
#[doc = "Field `PROCID` reader - Semaphore ProcessID"]
pub type ProcidR = crate::FieldReader;
#[doc = "Field `PROCID` writer - Semaphore ProcessID"]
pub type ProcidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COREID` reader - Semaphore COREID"]
pub type CoreidR = crate::FieldReader;
#[doc = "Field `COREID` writer - Semaphore COREID"]
pub type CoreidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LOCK` reader - Lock indication"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - Lock indication"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Semaphore ProcessID"]
    #[inline(always)]
    pub fn procid(&self) -> ProcidR {
        ProcidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Semaphore COREID"]
    #[inline(always)]
    pub fn coreid(&self) -> CoreidR {
        CoreidR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Lock indication"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Semaphore ProcessID"]
    #[inline(always)]
    pub fn procid(&mut self) -> ProcidW<HsemR20Spec> {
        ProcidW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Semaphore COREID"]
    #[inline(always)]
    pub fn coreid(&mut self) -> CoreidW<HsemR20Spec> {
        CoreidW::new(self, 8)
    }
    #[doc = "Bit 31 - Lock indication"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<HsemR20Spec> {
        LockW::new(self, 31)
    }
}
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsemR20Spec;
impl crate::RegisterSpec for HsemR20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_r20::R`](R) reader structure"]
impl crate::Readable for HsemR20Spec {}
#[doc = "`write(|w| ..)` method takes [`hsem_r20::W`](W) writer structure"]
impl crate::Writable for HsemR20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSEM_R20 to value 0"]
impl crate::Resettable for HsemR20Spec {}
