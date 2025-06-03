#[doc = "Register `CSGCMCCM4R` reader"]
pub type R = crate::R<Csgcmccm4rSpec>;
#[doc = "Register `CSGCMCCM4R` writer"]
pub type W = crate::W<Csgcmccm4rSpec>;
#[doc = "Field `CSGCMCCM4R` reader - CSGCMCCM4R"]
pub type Csgcmccm4rR = crate::FieldReader<u32>;
#[doc = "Field `CSGCMCCM4R` writer - CSGCMCCM4R"]
pub type Csgcmccm4rW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM4R"]
    #[inline(always)]
    pub fn csgcmccm4r(&self) -> Csgcmccm4rR {
        Csgcmccm4rR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM4R"]
    #[inline(always)]
    pub fn csgcmccm4r(&mut self) -> Csgcmccm4rW<Csgcmccm4rSpec> {
        Csgcmccm4rW::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`csgcmccm4r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm4r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csgcmccm4rSpec;
impl crate::RegisterSpec for Csgcmccm4rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcmccm4r::R`](R) reader structure"]
impl crate::Readable for Csgcmccm4rSpec {}
#[doc = "`write(|w| ..)` method takes [`csgcmccm4r::W`](W) writer structure"]
impl crate::Writable for Csgcmccm4rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSGCMCCM4R to value 0"]
impl crate::Resettable for Csgcmccm4rSpec {}
