#[doc = "Register `MDMA_C4DAR` reader"]
pub type R = crate::R<MdmaC4darSpec>;
#[doc = "Register `MDMA_C4DAR` writer"]
pub type W = crate::W<MdmaC4darSpec>;
#[doc = "Field `DAR` reader - Destination adr base"]
pub type DarR = crate::FieldReader<u32>;
#[doc = "Field `DAR` writer - Destination adr base"]
pub type DarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination adr base"]
    #[inline(always)]
    pub fn dar(&self) -> DarR {
        DarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination adr base"]
    #[inline(always)]
    pub fn dar(&mut self) -> DarW<MdmaC4darSpec> {
        DarW::new(self, 0)
    }
}
#[doc = "MDMA channel x destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c4dar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4dar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC4darSpec;
impl crate::RegisterSpec for MdmaC4darSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c4dar::R`](R) reader structure"]
impl crate::Readable for MdmaC4darSpec {}
#[doc = "`write(|w| ..)` method takes [`mdma_c4dar::W`](W) writer structure"]
impl crate::Writable for MdmaC4darSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C4DAR to value 0"]
impl crate::Resettable for MdmaC4darSpec {}
