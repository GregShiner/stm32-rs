#[doc = "Register `CSGCMCCM0R` reader"]
pub type R = crate::R<Csgcmccm0rSpec>;
#[doc = "Register `CSGCMCCM0R` writer"]
pub type W = crate::W<Csgcmccm0rSpec>;
#[doc = "Field `CSGCMCCM0R` reader - CSGCMCCM0R"]
pub type Csgcmccm0rR = crate::FieldReader<u32>;
#[doc = "Field `CSGCMCCM0R` writer - CSGCMCCM0R"]
pub type Csgcmccm0rW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM0R"]
    #[inline(always)]
    pub fn csgcmccm0r(&self) -> Csgcmccm0rR {
        Csgcmccm0rR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM0R"]
    #[inline(always)]
    pub fn csgcmccm0r(&mut self) -> Csgcmccm0rW<Csgcmccm0rSpec> {
        Csgcmccm0rW::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`csgcmccm0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csgcmccm0rSpec;
impl crate::RegisterSpec for Csgcmccm0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcmccm0r::R`](R) reader structure"]
impl crate::Readable for Csgcmccm0rSpec {}
#[doc = "`write(|w| ..)` method takes [`csgcmccm0r::W`](W) writer structure"]
impl crate::Writable for Csgcmccm0rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSGCMCCM0R to value 0"]
impl crate::Resettable for Csgcmccm0rSpec {}
