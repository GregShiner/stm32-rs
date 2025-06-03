#[doc = "Register `CSGCMCCM6R` reader"]
pub type R = crate::R<Csgcmccm6rSpec>;
#[doc = "Register `CSGCMCCM6R` writer"]
pub type W = crate::W<Csgcmccm6rSpec>;
#[doc = "Field `CSGCMCCM6R` reader - CSGCMCCM6R"]
pub type Csgcmccm6rR = crate::FieldReader<u32>;
#[doc = "Field `CSGCMCCM6R` writer - CSGCMCCM6R"]
pub type Csgcmccm6rW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM6R"]
    #[inline(always)]
    pub fn csgcmccm6r(&self) -> Csgcmccm6rR {
        Csgcmccm6rR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM6R"]
    #[inline(always)]
    pub fn csgcmccm6r(&mut self) -> Csgcmccm6rW<Csgcmccm6rSpec> {
        Csgcmccm6rW::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`csgcmccm6r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm6r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csgcmccm6rSpec;
impl crate::RegisterSpec for Csgcmccm6rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcmccm6r::R`](R) reader structure"]
impl crate::Readable for Csgcmccm6rSpec {}
#[doc = "`write(|w| ..)` method takes [`csgcmccm6r::W`](W) writer structure"]
impl crate::Writable for Csgcmccm6rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSGCMCCM6R to value 0"]
impl crate::Resettable for Csgcmccm6rSpec {}
