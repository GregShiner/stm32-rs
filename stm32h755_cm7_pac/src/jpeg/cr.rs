#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `JCEN` reader - JPEG Core Enable Enable the JPEG codec Core."]
pub type JcenR = crate::BitReader;
#[doc = "Field `JCEN` writer - JPEG Core Enable Enable the JPEG codec Core."]
pub type JcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFTIE` reader - Input FIFO Threshold Interrupt Enable This bit enables the interrupt generation when input FIFO reach the threshold."]
pub type IftieR = crate::BitReader;
#[doc = "Field `IFTIE` writer - Input FIFO Threshold Interrupt Enable This bit enables the interrupt generation when input FIFO reach the threshold."]
pub type IftieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFNFIE` reader - Input FIFO Not Full Interrupt Enable This bit enables the interrupt generation when input FIFO is not empty."]
pub type IfnfieR = crate::BitReader;
#[doc = "Field `IFNFIE` writer - Input FIFO Not Full Interrupt Enable This bit enables the interrupt generation when input FIFO is not empty."]
pub type IfnfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFTIE` reader - Output FIFO Threshold Interrupt Enable This bit enables the interrupt generation when output FIFO reach the threshold."]
pub type OftieR = crate::BitReader;
#[doc = "Field `OFTIE` writer - Output FIFO Threshold Interrupt Enable This bit enables the interrupt generation when output FIFO reach the threshold."]
pub type OftieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFNEIE` reader - Output FIFO Not Empty Interrupt Enable This bit enables the interrupt generation when output FIFO is not empty."]
pub type OfneieR = crate::BitReader;
#[doc = "Field `OFNEIE` writer - Output FIFO Not Empty Interrupt Enable This bit enables the interrupt generation when output FIFO is not empty."]
pub type OfneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCIE` reader - End of Conversion Interrupt Enable This bit enables the interrupt generation on the end of conversion."]
pub type EocieR = crate::BitReader;
#[doc = "Field `EOCIE` writer - End of Conversion Interrupt Enable This bit enables the interrupt generation on the end of conversion."]
pub type EocieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPDIE` reader - Header Parsing Done Interrupt Enable This bit enables the interrupt generation on the Header Parsing Operation."]
pub type HpdieR = crate::BitReader;
#[doc = "Field `HPDIE` writer - Header Parsing Done Interrupt Enable This bit enables the interrupt generation on the Header Parsing Operation."]
pub type HpdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDMAEN` reader - Input DMA Enable Enable the DMA request generation for the input FIFO."]
pub type IdmaenR = crate::BitReader;
#[doc = "Field `IDMAEN` writer - Input DMA Enable Enable the DMA request generation for the input FIFO."]
pub type IdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODMAEN` reader - Output DMA Enable Enable the DMA request generation for the output FIFO."]
pub type OdmaenR = crate::BitReader;
#[doc = "Field `ODMAEN` writer - Output DMA Enable Enable the DMA request generation for the output FIFO."]
pub type OdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFF` reader - Input FIFO Flush This bit flush the input FIFO. This bit is always read as 0."]
pub type IffR = crate::BitReader;
#[doc = "Field `IFF` writer - Input FIFO Flush This bit flush the input FIFO. This bit is always read as 0."]
pub type IffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFF` reader - Output FIFO Flush This bit flush the output FIFO. This bit is always read as 0."]
pub type OffR = crate::BitReader;
#[doc = "Field `OFF` writer - Output FIFO Flush This bit flush the output FIFO. This bit is always read as 0."]
pub type OffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - JPEG Core Enable Enable the JPEG codec Core."]
    #[inline(always)]
    pub fn jcen(&self) -> JcenR {
        JcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input FIFO Threshold Interrupt Enable This bit enables the interrupt generation when input FIFO reach the threshold."]
    #[inline(always)]
    pub fn iftie(&self) -> IftieR {
        IftieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input FIFO Not Full Interrupt Enable This bit enables the interrupt generation when input FIFO is not empty."]
    #[inline(always)]
    pub fn ifnfie(&self) -> IfnfieR {
        IfnfieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output FIFO Threshold Interrupt Enable This bit enables the interrupt generation when output FIFO reach the threshold."]
    #[inline(always)]
    pub fn oftie(&self) -> OftieR {
        OftieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output FIFO Not Empty Interrupt Enable This bit enables the interrupt generation when output FIFO is not empty."]
    #[inline(always)]
    pub fn ofneie(&self) -> OfneieR {
        OfneieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Enable This bit enables the interrupt generation on the end of conversion."]
    #[inline(always)]
    pub fn eocie(&self) -> EocieR {
        EocieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Header Parsing Done Interrupt Enable This bit enables the interrupt generation on the Header Parsing Operation."]
    #[inline(always)]
    pub fn hpdie(&self) -> HpdieR {
        HpdieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 11 - Input DMA Enable Enable the DMA request generation for the input FIFO."]
    #[inline(always)]
    pub fn idmaen(&self) -> IdmaenR {
        IdmaenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output DMA Enable Enable the DMA request generation for the output FIFO."]
    #[inline(always)]
    pub fn odmaen(&self) -> OdmaenR {
        OdmaenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input FIFO Flush This bit flush the input FIFO. This bit is always read as 0."]
    #[inline(always)]
    pub fn iff(&self) -> IffR {
        IffR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output FIFO Flush This bit flush the output FIFO. This bit is always read as 0."]
    #[inline(always)]
    pub fn off(&self) -> OffR {
        OffR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - JPEG Core Enable Enable the JPEG codec Core."]
    #[inline(always)]
    pub fn jcen(&mut self) -> JcenW<CrSpec> {
        JcenW::new(self, 0)
    }
    #[doc = "Bit 1 - Input FIFO Threshold Interrupt Enable This bit enables the interrupt generation when input FIFO reach the threshold."]
    #[inline(always)]
    pub fn iftie(&mut self) -> IftieW<CrSpec> {
        IftieW::new(self, 1)
    }
    #[doc = "Bit 2 - Input FIFO Not Full Interrupt Enable This bit enables the interrupt generation when input FIFO is not empty."]
    #[inline(always)]
    pub fn ifnfie(&mut self) -> IfnfieW<CrSpec> {
        IfnfieW::new(self, 2)
    }
    #[doc = "Bit 3 - Output FIFO Threshold Interrupt Enable This bit enables the interrupt generation when output FIFO reach the threshold."]
    #[inline(always)]
    pub fn oftie(&mut self) -> OftieW<CrSpec> {
        OftieW::new(self, 3)
    }
    #[doc = "Bit 4 - Output FIFO Not Empty Interrupt Enable This bit enables the interrupt generation when output FIFO is not empty."]
    #[inline(always)]
    pub fn ofneie(&mut self) -> OfneieW<CrSpec> {
        OfneieW::new(self, 4)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Enable This bit enables the interrupt generation on the end of conversion."]
    #[inline(always)]
    pub fn eocie(&mut self) -> EocieW<CrSpec> {
        EocieW::new(self, 5)
    }
    #[doc = "Bit 6 - Header Parsing Done Interrupt Enable This bit enables the interrupt generation on the Header Parsing Operation."]
    #[inline(always)]
    pub fn hpdie(&mut self) -> HpdieW<CrSpec> {
        HpdieW::new(self, 6)
    }
    #[doc = "Bit 11 - Input DMA Enable Enable the DMA request generation for the input FIFO."]
    #[inline(always)]
    pub fn idmaen(&mut self) -> IdmaenW<CrSpec> {
        IdmaenW::new(self, 11)
    }
    #[doc = "Bit 12 - Output DMA Enable Enable the DMA request generation for the output FIFO."]
    #[inline(always)]
    pub fn odmaen(&mut self) -> OdmaenW<CrSpec> {
        OdmaenW::new(self, 12)
    }
    #[doc = "Bit 13 - Input FIFO Flush This bit flush the input FIFO. This bit is always read as 0."]
    #[inline(always)]
    pub fn iff(&mut self) -> IffW<CrSpec> {
        IffW::new(self, 13)
    }
    #[doc = "Bit 14 - Output FIFO Flush This bit flush the output FIFO. This bit is always read as 0."]
    #[inline(always)]
    pub fn off(&mut self) -> OffW<CrSpec> {
        OffW::new(self, 14)
    }
}
#[doc = "JPEG control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
