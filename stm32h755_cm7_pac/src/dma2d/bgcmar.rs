#[doc = "Register `BGCMAR` reader"]
pub type R = crate::R<BgcmarSpec>;
#[doc = "Register `BGCMAR` writer"]
pub type W = crate::W<BgcmarSpec>;
#[doc = "Field `MA` reader - Memory address Address of the data used for the CLUT address dedicated to the background image. This register can only be written when no transfer is on going. Once the CLUT transfer has started, this register is read-only. If the background CLUT format is 32-bit, the address must be 32-bit aligned."]
pub type MaR = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - Memory address Address of the data used for the CLUT address dedicated to the background image. This register can only be written when no transfer is on going. Once the CLUT transfer has started, this register is read-only. If the background CLUT format is 32-bit, the address must be 32-bit aligned."]
pub type MaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory address Address of the data used for the CLUT address dedicated to the background image. This register can only be written when no transfer is on going. Once the CLUT transfer has started, this register is read-only. If the background CLUT format is 32-bit, the address must be 32-bit aligned."]
    #[inline(always)]
    pub fn ma(&self) -> MaR {
        MaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address Address of the data used for the CLUT address dedicated to the background image. This register can only be written when no transfer is on going. Once the CLUT transfer has started, this register is read-only. If the background CLUT format is 32-bit, the address must be 32-bit aligned."]
    #[inline(always)]
    pub fn ma(&mut self) -> MaW<BgcmarSpec> {
        MaW::new(self, 0)
    }
}
#[doc = "DMA2D background CLUT memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`bgcmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BgcmarSpec;
impl crate::RegisterSpec for BgcmarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bgcmar::R`](R) reader structure"]
impl crate::Readable for BgcmarSpec {}
#[doc = "`write(|w| ..)` method takes [`bgcmar::W`](W) writer structure"]
impl crate::Writable for BgcmarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BGCMAR to value 0"]
impl crate::Resettable for BgcmarSpec {}
