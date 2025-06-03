#[doc = "Register `CSGCM0R` reader"]
pub type R = crate::R<Csgcm0rSpec>;
#[doc = "Register `CSGCM0R` writer"]
pub type W = crate::W<Csgcm0rSpec>;
#[doc = "Field `CSGCM0R` reader - CSGCM0R"]
pub type Csgcm0rR = crate::FieldReader<u32>;
#[doc = "Field `CSGCM0R` writer - CSGCM0R"]
pub type Csgcm0rW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCM0R"]
    #[inline(always)]
    pub fn csgcm0r(&self) -> Csgcm0rR {
        Csgcm0rR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM0R"]
    #[inline(always)]
    pub fn csgcm0r(&mut self) -> Csgcm0rW<Csgcm0rSpec> {
        Csgcm0rW::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`csgcm0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csgcm0rSpec;
impl crate::RegisterSpec for Csgcm0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcm0r::R`](R) reader structure"]
impl crate::Readable for Csgcm0rSpec {}
#[doc = "`write(|w| ..)` method takes [`csgcm0r::W`](W) writer structure"]
impl crate::Writable for Csgcm0rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSGCM0R to value 0"]
impl crate::Resettable for Csgcm0rSpec {}
