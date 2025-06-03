#[doc = "Register `LWR` reader"]
pub type R = crate::R<LwrSpec>;
#[doc = "Register `LWR` writer"]
pub type W = crate::W<LwrSpec>;
#[doc = "Field `LW` reader - Line watermark These bits allow to configure the line watermark for interrupt generation. An interrupt is raised when the last pixel of the watermarked line has been transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type LwR = crate::FieldReader<u16>;
#[doc = "Field `LW` writer - Line watermark These bits allow to configure the line watermark for interrupt generation. An interrupt is raised when the last pixel of the watermarked line has been transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type LwW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Line watermark These bits allow to configure the line watermark for interrupt generation. An interrupt is raised when the last pixel of the watermarked line has been transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn lw(&self) -> LwR {
        LwR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Line watermark These bits allow to configure the line watermark for interrupt generation. An interrupt is raised when the last pixel of the watermarked line has been transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn lw(&mut self) -> LwW<LwrSpec> {
        LwW::new(self, 0)
    }
}
#[doc = "DMA2D line watermark register\n\nYou can [`read`](crate::Reg::read) this register and get [`lwr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lwr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LwrSpec;
impl crate::RegisterSpec for LwrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lwr::R`](R) reader structure"]
impl crate::Readable for LwrSpec {}
#[doc = "`write(|w| ..)` method takes [`lwr::W`](W) writer structure"]
impl crate::Writable for LwrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LWR to value 0"]
impl crate::Resettable for LwrSpec {}
