#[doc = "Register `PSMAR` reader"]
pub type R = crate::R<PsmarSpec>;
#[doc = "Register `PSMAR` writer"]
pub type W = crate::W<PsmarSpec>;
#[doc = "Field `MATCH` reader - Status match Value to be compared with the masked status register to get a match. This field can be written only when BUSY = 0."]
pub type MatchR = crate::FieldReader<u32>;
#[doc = "Field `MATCH` writer - Status match Value to be compared with the masked status register to get a match. This field can be written only when BUSY = 0."]
pub type MatchW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Status match Value to be compared with the masked status register to get a match. This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn match_(&self) -> MatchR {
        MatchR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Status match Value to be compared with the masked status register to get a match. This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn match_(&mut self) -> MatchW<PsmarSpec> {
        MatchW::new(self, 0)
    }
}
#[doc = "QUADSPI polling status match register\n\nYou can [`read`](crate::Reg::read) this register and get [`psmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsmarSpec;
impl crate::RegisterSpec for PsmarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psmar::R`](R) reader structure"]
impl crate::Readable for PsmarSpec {}
#[doc = "`write(|w| ..)` method takes [`psmar::W`](W) writer structure"]
impl crate::Writable for PsmarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSMAR to value 0"]
impl crate::Resettable for PsmarSpec {}
