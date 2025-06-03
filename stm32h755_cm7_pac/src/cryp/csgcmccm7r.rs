#[doc = "Register `CSGCMCCM7R` reader"]
pub type R = crate::R<Csgcmccm7rSpec>;
#[doc = "Register `CSGCMCCM7R` writer"]
pub type W = crate::W<Csgcmccm7rSpec>;
#[doc = "Field `CSGCMCCM7R` reader - CSGCMCCM7R"]
pub type Csgcmccm7rR = crate::FieldReader<u32>;
#[doc = "Field `CSGCMCCM7R` writer - CSGCMCCM7R"]
pub type Csgcmccm7rW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM7R"]
    #[inline(always)]
    pub fn csgcmccm7r(&self) -> Csgcmccm7rR {
        Csgcmccm7rR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM7R"]
    #[inline(always)]
    pub fn csgcmccm7r(&mut self) -> Csgcmccm7rW<Csgcmccm7rSpec> {
        Csgcmccm7rW::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`csgcmccm7r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm7r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csgcmccm7rSpec;
impl crate::RegisterSpec for Csgcmccm7rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcmccm7r::R`](R) reader structure"]
impl crate::Readable for Csgcmccm7rSpec {}
#[doc = "`write(|w| ..)` method takes [`csgcmccm7r::W`](W) writer structure"]
impl crate::Writable for Csgcmccm7rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSGCMCCM7R to value 0"]
impl crate::Resettable for Csgcmccm7rSpec {}
