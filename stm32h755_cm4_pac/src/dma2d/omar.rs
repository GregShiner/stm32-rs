#[doc = "Register `OMAR` reader"]
pub type R = crate::R<OmarSpec>;
#[doc = "Register `OMAR` writer"]
pub type W = crate::W<OmarSpec>;
#[doc = "Field `MA` reader - Memory Address Address of the data used for the output FIFO. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned and a 16-bit per pixel format must be 16-bit aligned."]
pub type MaR = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - Memory Address Address of the data used for the output FIFO. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned and a 16-bit per pixel format must be 16-bit aligned."]
pub type MaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory Address Address of the data used for the output FIFO. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned and a 16-bit per pixel format must be 16-bit aligned."]
    #[inline(always)]
    pub fn ma(&self) -> MaR {
        MaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory Address Address of the data used for the output FIFO. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned and a 16-bit per pixel format must be 16-bit aligned."]
    #[inline(always)]
    pub fn ma(&mut self) -> MaW<OmarSpec> {
        MaW::new(self, 0)
    }
}
#[doc = "DMA2D output memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`omar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`omar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OmarSpec;
impl crate::RegisterSpec for OmarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`omar::R`](R) reader structure"]
impl crate::Readable for OmarSpec {}
#[doc = "`write(|w| ..)` method takes [`omar::W`](W) writer structure"]
impl crate::Writable for OmarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OMAR to value 0"]
impl crate::Resettable for OmarSpec {}
