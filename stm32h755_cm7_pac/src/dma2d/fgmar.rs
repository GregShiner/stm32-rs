#[doc = "Register `FGMAR` reader"]
pub type R = crate::R<FgmarSpec>;
#[doc = "Register `FGMAR` writer"]
pub type W = crate::W<FgmarSpec>;
#[doc = "Field `MA` reader - Memory address Address of the data used for the foreground image. This register can only be written when data transfers are disabled. Once the data transfer has started, this register is read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned, a 16-bit per pixel format must be 16-bit aligned and a 4-bit per pixel format must be 8-bit aligned."]
pub type MaR = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - Memory address Address of the data used for the foreground image. This register can only be written when data transfers are disabled. Once the data transfer has started, this register is read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned, a 16-bit per pixel format must be 16-bit aligned and a 4-bit per pixel format must be 8-bit aligned."]
pub type MaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory address Address of the data used for the foreground image. This register can only be written when data transfers are disabled. Once the data transfer has started, this register is read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned, a 16-bit per pixel format must be 16-bit aligned and a 4-bit per pixel format must be 8-bit aligned."]
    #[inline(always)]
    pub fn ma(&self) -> MaR {
        MaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address Address of the data used for the foreground image. This register can only be written when data transfers are disabled. Once the data transfer has started, this register is read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned, a 16-bit per pixel format must be 16-bit aligned and a 4-bit per pixel format must be 8-bit aligned."]
    #[inline(always)]
    pub fn ma(&mut self) -> MaW<FgmarSpec> {
        MaW::new(self, 0)
    }
}
#[doc = "DMA2D foreground memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`fgmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FgmarSpec;
impl crate::RegisterSpec for FgmarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fgmar::R`](R) reader structure"]
impl crate::Readable for FgmarSpec {}
#[doc = "`write(|w| ..)` method takes [`fgmar::W`](W) writer structure"]
impl crate::Writable for FgmarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FGMAR to value 0"]
impl crate::Resettable for FgmarSpec {}
