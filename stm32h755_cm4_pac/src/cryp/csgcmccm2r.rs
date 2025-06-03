#[doc = "Register `CSGCMCCM2R` reader"]
pub type R = crate::R<Csgcmccm2rSpec>;
#[doc = "Register `CSGCMCCM2R` writer"]
pub type W = crate::W<Csgcmccm2rSpec>;
#[doc = "Field `CSGCMCCM2R` reader - CSGCMCCM2R"]
pub type Csgcmccm2rR = crate::FieldReader<u32>;
#[doc = "Field `CSGCMCCM2R` writer - CSGCMCCM2R"]
pub type Csgcmccm2rW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM2R"]
    #[inline(always)]
    pub fn csgcmccm2r(&self) -> Csgcmccm2rR {
        Csgcmccm2rR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM2R"]
    #[inline(always)]
    pub fn csgcmccm2r(&mut self) -> Csgcmccm2rW<Csgcmccm2rSpec> {
        Csgcmccm2rW::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`csgcmccm2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csgcmccm2rSpec;
impl crate::RegisterSpec for Csgcmccm2rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcmccm2r::R`](R) reader structure"]
impl crate::Readable for Csgcmccm2rSpec {}
#[doc = "`write(|w| ..)` method takes [`csgcmccm2r::W`](W) writer structure"]
impl crate::Writable for Csgcmccm2rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSGCMCCM2R to value 0"]
impl crate::Resettable for Csgcmccm2rSpec {}
