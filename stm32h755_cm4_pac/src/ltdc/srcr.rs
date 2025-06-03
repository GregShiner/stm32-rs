#[doc = "Register `SRCR` reader"]
pub type R = crate::R<SrcrSpec>;
#[doc = "Register `SRCR` writer"]
pub type W = crate::W<SrcrSpec>;
#[doc = "Field `IMR` reader - Immediate Reload"]
pub type ImrR = crate::BitReader;
#[doc = "Field `IMR` writer - Immediate Reload"]
pub type ImrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBR` reader - Vertical Blanking Reload"]
pub type VbrR = crate::BitReader;
#[doc = "Field `VBR` writer - Vertical Blanking Reload"]
pub type VbrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Immediate Reload"]
    #[inline(always)]
    pub fn imr(&self) -> ImrR {
        ImrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Vertical Blanking Reload"]
    #[inline(always)]
    pub fn vbr(&self) -> VbrR {
        VbrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Immediate Reload"]
    #[inline(always)]
    pub fn imr(&mut self) -> ImrW<SrcrSpec> {
        ImrW::new(self, 0)
    }
    #[doc = "Bit 1 - Vertical Blanking Reload"]
    #[inline(always)]
    pub fn vbr(&mut self) -> VbrW<SrcrSpec> {
        VbrW::new(self, 1)
    }
}
#[doc = "Shadow Reload Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcrSpec;
impl crate::RegisterSpec for SrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcr::R`](R) reader structure"]
impl crate::Readable for SrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`srcr::W`](W) writer structure"]
impl crate::Writable for SrcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRCR to value 0"]
impl crate::Resettable for SrcrSpec {}
