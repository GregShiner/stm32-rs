#[doc = "Register `NLR` reader"]
pub type R = crate::R<NlrSpec>;
#[doc = "Register `NLR` writer"]
pub type W = crate::W<NlrSpec>;
#[doc = "Field `NL` reader - Number of lines Number of lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type NlR = crate::FieldReader<u16>;
#[doc = "Field `NL` writer - Number of lines Number of lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type NlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PL` reader - Pixel per lines Number of pixels per lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. If any of the input image format is 4-bit per pixel, pixel per lines must be even."]
pub type PlR = crate::FieldReader<u16>;
#[doc = "Field `PL` writer - Pixel per lines Number of pixels per lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. If any of the input image format is 4-bit per pixel, pixel per lines must be even."]
pub type PlW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of lines Number of lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn nl(&self) -> NlR {
        NlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:29 - Pixel per lines Number of pixels per lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. If any of the input image format is 4-bit per pixel, pixel per lines must be even."]
    #[inline(always)]
    pub fn pl(&self) -> PlR {
        PlR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of lines Number of lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn nl(&mut self) -> NlW<NlrSpec> {
        NlW::new(self, 0)
    }
    #[doc = "Bits 16:29 - Pixel per lines Number of pixels per lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. If any of the input image format is 4-bit per pixel, pixel per lines must be even."]
    #[inline(always)]
    pub fn pl(&mut self) -> PlW<NlrSpec> {
        PlW::new(self, 16)
    }
}
#[doc = "DMA2D number of line register\n\nYou can [`read`](crate::Reg::read) this register and get [`nlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NlrSpec;
impl crate::RegisterSpec for NlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nlr::R`](R) reader structure"]
impl crate::Readable for NlrSpec {}
#[doc = "`write(|w| ..)` method takes [`nlr::W`](W) writer structure"]
impl crate::Writable for NlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NLR to value 0"]
impl crate::Resettable for NlrSpec {}
