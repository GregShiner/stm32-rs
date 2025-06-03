#[doc = "Register `SAI_GCR` reader"]
pub type R = crate::R<SaiGcrSpec>;
#[doc = "Register `SAI_GCR` writer"]
pub type W = crate::W<SaiGcrSpec>;
#[doc = "Field `SYNCIN` reader - Synchronization inputs"]
pub type SyncinR = crate::FieldReader;
#[doc = "Field `SYNCIN` writer - Synchronization inputs"]
pub type SyncinW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYNCOUT` reader - Synchronization outputs These bits are set and cleared by software."]
pub type SyncoutR = crate::FieldReader;
#[doc = "Field `SYNCOUT` writer - Synchronization outputs These bits are set and cleared by software."]
pub type SyncoutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Synchronization inputs"]
    #[inline(always)]
    pub fn syncin(&self) -> SyncinR {
        SyncinR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Synchronization outputs These bits are set and cleared by software."]
    #[inline(always)]
    pub fn syncout(&self) -> SyncoutR {
        SyncoutR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Synchronization inputs"]
    #[inline(always)]
    pub fn syncin(&mut self) -> SyncinW<SaiGcrSpec> {
        SyncinW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Synchronization outputs These bits are set and cleared by software."]
    #[inline(always)]
    pub fn syncout(&mut self) -> SyncoutW<SaiGcrSpec> {
        SyncoutW::new(self, 4)
    }
}
#[doc = "Global configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sai_gcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sai_gcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaiGcrSpec;
impl crate::RegisterSpec for SaiGcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sai_gcr::R`](R) reader structure"]
impl crate::Readable for SaiGcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sai_gcr::W`](W) writer structure"]
impl crate::Writable for SaiGcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAI_GCR to value 0"]
impl crate::Resettable for SaiGcrSpec {}
