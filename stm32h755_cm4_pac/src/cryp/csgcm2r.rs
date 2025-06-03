#[doc = "Register `CSGCM2R` reader"]
pub type R = crate::R<Csgcm2rSpec>;
#[doc = "Register `CSGCM2R` writer"]
pub type W = crate::W<Csgcm2rSpec>;
#[doc = "Field `CSGCM2R` reader - CSGCM2R"]
pub type Csgcm2rR = crate::FieldReader<u32>;
#[doc = "Field `CSGCM2R` writer - CSGCM2R"]
pub type Csgcm2rW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCM2R"]
    #[inline(always)]
    pub fn csgcm2r(&self) -> Csgcm2rR {
        Csgcm2rR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM2R"]
    #[inline(always)]
    pub fn csgcm2r(&mut self) -> Csgcm2rW<Csgcm2rSpec> {
        Csgcm2rW::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`csgcm2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csgcm2rSpec;
impl crate::RegisterSpec for Csgcm2rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcm2r::R`](R) reader structure"]
impl crate::Readable for Csgcm2rSpec {}
#[doc = "`write(|w| ..)` method takes [`csgcm2r::W`](W) writer structure"]
impl crate::Writable for Csgcm2rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSGCM2R to value 0"]
impl crate::Resettable for Csgcm2rSpec {}
