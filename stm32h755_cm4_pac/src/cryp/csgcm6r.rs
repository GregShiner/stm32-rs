#[doc = "Register `CSGCM6R` reader"]
pub type R = crate::R<Csgcm6rSpec>;
#[doc = "Register `CSGCM6R` writer"]
pub type W = crate::W<Csgcm6rSpec>;
#[doc = "Field `CSGCM6R` reader - CSGCM6R"]
pub type Csgcm6rR = crate::FieldReader<u32>;
#[doc = "Field `CSGCM6R` writer - CSGCM6R"]
pub type Csgcm6rW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCM6R"]
    #[inline(always)]
    pub fn csgcm6r(&self) -> Csgcm6rR {
        Csgcm6rR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM6R"]
    #[inline(always)]
    pub fn csgcm6r(&mut self) -> Csgcm6rW<Csgcm6rSpec> {
        Csgcm6rW::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`csgcm6r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm6r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csgcm6rSpec;
impl crate::RegisterSpec for Csgcm6rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcm6r::R`](R) reader structure"]
impl crate::Readable for Csgcm6rSpec {}
#[doc = "`write(|w| ..)` method takes [`csgcm6r::W`](W) writer structure"]
impl crate::Writable for Csgcm6rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSGCM6R to value 0"]
impl crate::Resettable for Csgcm6rSpec {}
