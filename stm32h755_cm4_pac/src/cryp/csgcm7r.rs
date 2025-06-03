#[doc = "Register `CSGCM7R` reader"]
pub type R = crate::R<Csgcm7rSpec>;
#[doc = "Register `CSGCM7R` writer"]
pub type W = crate::W<Csgcm7rSpec>;
#[doc = "Field `CSGCM7R` reader - CSGCM7R"]
pub type Csgcm7rR = crate::FieldReader<u32>;
#[doc = "Field `CSGCM7R` writer - CSGCM7R"]
pub type Csgcm7rW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCM7R"]
    #[inline(always)]
    pub fn csgcm7r(&self) -> Csgcm7rR {
        Csgcm7rR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM7R"]
    #[inline(always)]
    pub fn csgcm7r(&mut self) -> Csgcm7rW<Csgcm7rSpec> {
        Csgcm7rW::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`csgcm7r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm7r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csgcm7rSpec;
impl crate::RegisterSpec for Csgcm7rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcm7r::R`](R) reader structure"]
impl crate::Readable for Csgcm7rSpec {}
#[doc = "`write(|w| ..)` method takes [`csgcm7r::W`](W) writer structure"]
impl crate::Writable for Csgcm7rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSGCM7R to value 0"]
impl crate::Resettable for Csgcm7rSpec {}
