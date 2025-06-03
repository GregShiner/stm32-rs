#[doc = "Register `FGCMAR` reader"]
pub type R = crate::R<FgcmarSpec>;
#[doc = "Register `FGCMAR` writer"]
pub type W = crate::W<FgcmarSpec>;
#[doc = "Field `MA` reader - Memory Address Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only. If the foreground CLUT format is 32-bit, the address must be 32-bit aligned."]
pub type MaR = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - Memory Address Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only. If the foreground CLUT format is 32-bit, the address must be 32-bit aligned."]
pub type MaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory Address Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only. If the foreground CLUT format is 32-bit, the address must be 32-bit aligned."]
    #[inline(always)]
    pub fn ma(&self) -> MaR {
        MaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory Address Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only. If the foreground CLUT format is 32-bit, the address must be 32-bit aligned."]
    #[inline(always)]
    pub fn ma(&mut self) -> MaW<FgcmarSpec> {
        MaW::new(self, 0)
    }
}
#[doc = "DMA2D foreground CLUT memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`fgcmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgcmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FgcmarSpec;
impl crate::RegisterSpec for FgcmarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fgcmar::R`](R) reader structure"]
impl crate::Readable for FgcmarSpec {}
#[doc = "`write(|w| ..)` method takes [`fgcmar::W`](W) writer structure"]
impl crate::Writable for FgcmarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FGCMAR to value 0"]
impl crate::Resettable for FgcmarSpec {}
