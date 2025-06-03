#[doc = "Register `CSGCMCCM3R` reader"]
pub type R = crate::R<Csgcmccm3rSpec>;
#[doc = "Register `CSGCMCCM3R` writer"]
pub type W = crate::W<Csgcmccm3rSpec>;
#[doc = "Field `CSGCMCCM3R` reader - CSGCMCCM3R"]
pub type Csgcmccm3rR = crate::FieldReader<u32>;
#[doc = "Field `CSGCMCCM3R` writer - CSGCMCCM3R"]
pub type Csgcmccm3rW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM3R"]
    #[inline(always)]
    pub fn csgcmccm3r(&self) -> Csgcmccm3rR {
        Csgcmccm3rR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM3R"]
    #[inline(always)]
    pub fn csgcmccm3r(&mut self) -> Csgcmccm3rW<Csgcmccm3rSpec> {
        Csgcmccm3rW::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`csgcmccm3r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm3r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csgcmccm3rSpec;
impl crate::RegisterSpec for Csgcmccm3rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcmccm3r::R`](R) reader structure"]
impl crate::Readable for Csgcmccm3rSpec {}
#[doc = "`write(|w| ..)` method takes [`csgcmccm3r::W`](W) writer structure"]
impl crate::Writable for Csgcmccm3rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSGCMCCM3R to value 0"]
impl crate::Resettable for Csgcmccm3rSpec {}
