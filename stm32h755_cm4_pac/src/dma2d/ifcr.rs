#[doc = "Register `IFCR` reader"]
pub type R = crate::R<IfcrSpec>;
#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IfcrSpec>;
#[doc = "Field `CTEIF` reader - Clear Transfer error interrupt flag Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register"]
pub type CteifR = crate::BitReader;
#[doc = "Field `CTEIF` writer - Clear Transfer error interrupt flag Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register"]
pub type CteifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF` reader - Clear transfer complete interrupt flag Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register"]
pub type CtcifR = crate::BitReader;
#[doc = "Field `CTCIF` writer - Clear transfer complete interrupt flag Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register"]
pub type CtcifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTWIF` reader - Clear transfer watermark interrupt flag Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register"]
pub type CtwifR = crate::BitReader;
#[doc = "Field `CTWIF` writer - Clear transfer watermark interrupt flag Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register"]
pub type CtwifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAECIF` reader - Clear CLUT access error interrupt flag Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register"]
pub type CaecifR = crate::BitReader;
#[doc = "Field `CAECIF` writer - Clear CLUT access error interrupt flag Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register"]
pub type CaecifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCTCIF` reader - Clear CLUT transfer complete interrupt flag Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register"]
pub type CctcifR = crate::BitReader;
#[doc = "Field `CCTCIF` writer - Clear CLUT transfer complete interrupt flag Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register"]
pub type CctcifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCEIF` reader - Clear configuration error interrupt flag Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register"]
pub type CceifR = crate::BitReader;
#[doc = "Field `CCEIF` writer - Clear configuration error interrupt flag Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register"]
pub type CceifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear Transfer error interrupt flag Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn cteif(&self) -> CteifR {
        CteifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear transfer complete interrupt flag Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn ctcif(&self) -> CtcifR {
        CtcifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear transfer watermark interrupt flag Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn ctwif(&self) -> CtwifR {
        CtwifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear CLUT access error interrupt flag Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn caecif(&self) -> CaecifR {
        CaecifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear CLUT transfer complete interrupt flag Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn cctcif(&self) -> CctcifR {
        CctcifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear configuration error interrupt flag Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn cceif(&self) -> CceifR {
        CceifR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Transfer error interrupt flag Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn cteif(&mut self) -> CteifW<IfcrSpec> {
        CteifW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear transfer complete interrupt flag Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn ctcif(&mut self) -> CtcifW<IfcrSpec> {
        CtcifW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear transfer watermark interrupt flag Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn ctwif(&mut self) -> CtwifW<IfcrSpec> {
        CtwifW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear CLUT access error interrupt flag Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn caecif(&mut self) -> CaecifW<IfcrSpec> {
        CaecifW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear CLUT transfer complete interrupt flag Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn cctcif(&mut self) -> CctcifW<IfcrSpec> {
        CctcifW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear configuration error interrupt flag Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn cceif(&mut self) -> CceifW<IfcrSpec> {
        CceifW::new(self, 5)
    }
}
#[doc = "DMA2D interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcrSpec;
impl crate::RegisterSpec for IfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifcr::R`](R) reader structure"]
impl crate::Readable for IfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ifcr::W`](W) writer structure"]
impl crate::Writable for IfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IfcrSpec {}
