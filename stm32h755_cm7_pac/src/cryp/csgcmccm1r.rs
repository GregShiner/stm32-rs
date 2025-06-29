#[doc = "Register `CSGCMCCM1R` reader"]
pub type R = crate::R<Csgcmccm1rSpec>;
#[doc = "Register `CSGCMCCM1R` writer"]
pub type W = crate::W<Csgcmccm1rSpec>;
#[doc = "Field `CSGCMCCM1R` reader - CSGCMCCM1R"]
pub type Csgcmccm1rR = crate::FieldReader<u32>;
#[doc = "Field `CSGCMCCM1R` writer - CSGCMCCM1R"]
pub type Csgcmccm1rW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM1R"]
    #[inline(always)]
    pub fn csgcmccm1r(&self) -> Csgcmccm1rR {
        Csgcmccm1rR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM1R"]
    #[inline(always)]
    pub fn csgcmccm1r(&mut self) -> Csgcmccm1rW<Csgcmccm1rSpec> {
        Csgcmccm1rW::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`csgcmccm1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csgcmccm1rSpec;
impl crate::RegisterSpec for Csgcmccm1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcmccm1r::R`](R) reader structure"]
impl crate::Readable for Csgcmccm1rSpec {}
#[doc = "`write(|w| ..)` method takes [`csgcmccm1r::W`](W) writer structure"]
impl crate::Writable for Csgcmccm1rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSGCMCCM1R to value 0"]
impl crate::Resettable for Csgcmccm1rSpec {}
