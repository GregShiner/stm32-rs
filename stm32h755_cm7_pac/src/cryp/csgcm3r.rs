#[doc = "Register `CSGCM3R` reader"]
pub type R = crate::R<Csgcm3rSpec>;
#[doc = "Register `CSGCM3R` writer"]
pub type W = crate::W<Csgcm3rSpec>;
#[doc = "Field `CSGCM3R` reader - CSGCM3R"]
pub type Csgcm3rR = crate::FieldReader<u32>;
#[doc = "Field `CSGCM3R` writer - CSGCM3R"]
pub type Csgcm3rW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCM3R"]
    #[inline(always)]
    pub fn csgcm3r(&self) -> Csgcm3rR {
        Csgcm3rR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM3R"]
    #[inline(always)]
    pub fn csgcm3r(&mut self) -> Csgcm3rW<Csgcm3rSpec> {
        Csgcm3rW::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`csgcm3r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm3r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csgcm3rSpec;
impl crate::RegisterSpec for Csgcm3rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcm3r::R`](R) reader structure"]
impl crate::Readable for Csgcm3rSpec {}
#[doc = "`write(|w| ..)` method takes [`csgcm3r::W`](W) writer structure"]
impl crate::Writable for Csgcm3rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSGCM3R to value 0"]
impl crate::Resettable for Csgcm3rSpec {}
