#[doc = "Register `PSMKR` reader"]
pub type R = crate::R<PsmkrSpec>;
#[doc = "Register `PSMKR` writer"]
pub type W = crate::W<PsmkrSpec>;
#[doc = "Field `MASK` reader - Status mask Mask to be applied to the status bytes received in polling mode. For bit n: This field can be written only when BUSY = 0."]
pub type MaskR = crate::FieldReader<u32>;
#[doc = "Field `MASK` writer - Status mask Mask to be applied to the status bytes received in polling mode. For bit n: This field can be written only when BUSY = 0."]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Status mask Mask to be applied to the status bytes received in polling mode. For bit n: This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Status mask Mask to be applied to the status bytes received in polling mode. For bit n: This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<PsmkrSpec> {
        MaskW::new(self, 0)
    }
}
#[doc = "QUADSPI polling status mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`psmkr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmkr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsmkrSpec;
impl crate::RegisterSpec for PsmkrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psmkr::R`](R) reader structure"]
impl crate::Readable for PsmkrSpec {}
#[doc = "`write(|w| ..)` method takes [`psmkr::W`](W) writer structure"]
impl crate::Writable for PsmkrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSMKR to value 0"]
impl crate::Resettable for PsmkrSpec {}
