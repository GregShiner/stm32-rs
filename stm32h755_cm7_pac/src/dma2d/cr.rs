#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `START` reader - Start This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Start This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSP` reader - Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset."]
pub type SuspR = crate::BitReader;
#[doc = "Field `SUSP` writer - Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset."]
pub type SuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT` reader - Abort This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset."]
pub type AbortR = crate::BitReader;
#[doc = "Field `ABORT` writer - Abort This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset."]
pub type AbortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIE` reader - Transfer error interrupt enable This bit is set and cleared by software."]
pub type TeieR = crate::BitReader;
#[doc = "Field `TEIE` writer - Transfer error interrupt enable This bit is set and cleared by software."]
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable This bit is set and cleared by software."]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable This bit is set and cleared by software."]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWIE` reader - Transfer watermark interrupt enable This bit is set and cleared by software."]
pub type TwieR = crate::BitReader;
#[doc = "Field `TWIE` writer - Transfer watermark interrupt enable This bit is set and cleared by software."]
pub type TwieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAEIE` reader - CLUT access error interrupt enable This bit is set and cleared by software."]
pub type CaeieR = crate::BitReader;
#[doc = "Field `CAEIE` writer - CLUT access error interrupt enable This bit is set and cleared by software."]
pub type CaeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIE` reader - CLUT transfer complete interrupt enable This bit is set and cleared by software."]
pub type CtcieR = crate::BitReader;
#[doc = "Field `CTCIE` writer - CLUT transfer complete interrupt enable This bit is set and cleared by software."]
pub type CtcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEIE` reader - Configuration Error Interrupt Enable This bit is set and cleared by software."]
pub type CeieR = crate::BitReader;
#[doc = "Field `CEIE` writer - Configuration Error Interrupt Enable This bit is set and cleared by software."]
pub type CeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing."]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Start This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset."]
    #[inline(always)]
    pub fn susp(&self) -> SuspR {
        SuspR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Abort This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset."]
    #[inline(always)]
    pub fn abort(&self) -> AbortR {
        AbortR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Transfer error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transfer complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transfer watermark interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn twie(&self) -> TwieR {
        TwieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CLUT access error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn caeie(&self) -> CaeieR {
        CaeieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CLUT transfer complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ctcie(&self) -> CtcieR {
        CtcieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configuration Error Interrupt Enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ceie(&self) -> CeieR {
        CeieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<CrSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset."]
    #[inline(always)]
    pub fn susp(&mut self) -> SuspW<CrSpec> {
        SuspW::new(self, 1)
    }
    #[doc = "Bit 2 - Abort This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset."]
    #[inline(always)]
    pub fn abort(&mut self) -> AbortW<CrSpec> {
        AbortW::new(self, 2)
    }
    #[doc = "Bit 8 - Transfer error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn teie(&mut self) -> TeieW<CrSpec> {
        TeieW::new(self, 8)
    }
    #[doc = "Bit 9 - Transfer complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<CrSpec> {
        TcieW::new(self, 9)
    }
    #[doc = "Bit 10 - Transfer watermark interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn twie(&mut self) -> TwieW<CrSpec> {
        TwieW::new(self, 10)
    }
    #[doc = "Bit 11 - CLUT access error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn caeie(&mut self) -> CaeieW<CrSpec> {
        CaeieW::new(self, 11)
    }
    #[doc = "Bit 12 - CLUT transfer complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ctcie(&mut self) -> CtcieW<CrSpec> {
        CtcieW::new(self, 12)
    }
    #[doc = "Bit 13 - Configuration Error Interrupt Enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ceie(&mut self) -> CeieW<CrSpec> {
        CeieW::new(self, 13)
    }
    #[doc = "Bits 16:17 - DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<CrSpec> {
        ModeW::new(self, 16)
    }
}
#[doc = "DMA2D control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
