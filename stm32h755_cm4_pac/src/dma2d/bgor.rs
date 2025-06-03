#[doc = "Register `BGOR` reader"]
pub type R = crate::R<BgorSpec>;
#[doc = "Register `BGOR` writer"]
pub type W = crate::W<BgorSpec>;
#[doc = "Field `LO` reader - Line offset Line offset used for the background image (expressed in pixel). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
pub type LoR = crate::FieldReader<u16>;
#[doc = "Field `LO` writer - Line offset Line offset used for the background image (expressed in pixel). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
pub type LoW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Line offset Line offset used for the background image (expressed in pixel). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
    #[inline(always)]
    pub fn lo(&self) -> LoR {
        LoR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Line offset Line offset used for the background image (expressed in pixel). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
    #[inline(always)]
    pub fn lo(&mut self) -> LoW<BgorSpec> {
        LoW::new(self, 0)
    }
}
#[doc = "DMA2D background offset register\n\nYou can [`read`](crate::Reg::read) this register and get [`bgor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BgorSpec;
impl crate::RegisterSpec for BgorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bgor::R`](R) reader structure"]
impl crate::Readable for BgorSpec {}
#[doc = "`write(|w| ..)` method takes [`bgor::W`](W) writer structure"]
impl crate::Writable for BgorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BGOR to value 0"]
impl crate::Resettable for BgorSpec {}
