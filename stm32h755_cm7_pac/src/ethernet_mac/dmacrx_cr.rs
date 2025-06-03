#[doc = "Register `DMACRxCR` reader"]
pub type R = crate::R<DmacrxCrSpec>;
#[doc = "Register `DMACRxCR` writer"]
pub type W = crate::W<DmacrxCrSpec>;
#[doc = "Field `SR` reader - Start or Stop Receive Command"]
pub type SrR = crate::BitReader;
#[doc = "Field `SR` writer - Start or Stop Receive Command"]
pub type SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBSZ` reader - Receive Buffer size"]
pub type RbszR = crate::FieldReader<u16>;
#[doc = "Field `RBSZ` writer - Receive Buffer size"]
pub type RbszW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `RXPBL` reader - RXPBL"]
pub type RxpblR = crate::FieldReader;
#[doc = "Field `RXPBL` writer - RXPBL"]
pub type RxpblW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RPF` reader - DMA Rx Channel Packet Flush"]
pub type RpfR = crate::BitReader;
#[doc = "Field `RPF` writer - DMA Rx Channel Packet Flush"]
pub type RpfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start or Stop Receive Command"]
    #[inline(always)]
    pub fn sr(&self) -> SrR {
        SrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:14 - Receive Buffer size"]
    #[inline(always)]
    pub fn rbsz(&self) -> RbszR {
        RbszR::new(((self.bits >> 1) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - RXPBL"]
    #[inline(always)]
    pub fn rxpbl(&self) -> RxpblR {
        RxpblR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - DMA Rx Channel Packet Flush"]
    #[inline(always)]
    pub fn rpf(&self) -> RpfR {
        RpfR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start or Stop Receive Command"]
    #[inline(always)]
    pub fn sr(&mut self) -> SrW<DmacrxCrSpec> {
        SrW::new(self, 0)
    }
    #[doc = "Bits 1:14 - Receive Buffer size"]
    #[inline(always)]
    pub fn rbsz(&mut self) -> RbszW<DmacrxCrSpec> {
        RbszW::new(self, 1)
    }
    #[doc = "Bits 16:21 - RXPBL"]
    #[inline(always)]
    pub fn rxpbl(&mut self) -> RxpblW<DmacrxCrSpec> {
        RxpblW::new(self, 16)
    }
    #[doc = "Bit 31 - DMA Rx Channel Packet Flush"]
    #[inline(always)]
    pub fn rpf(&mut self) -> RpfW<DmacrxCrSpec> {
        RpfW::new(self, 31)
    }
}
#[doc = "Channel receive control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacrx_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrx_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacrxCrSpec;
impl crate::RegisterSpec for DmacrxCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrx_cr::R`](R) reader structure"]
impl crate::Readable for DmacrxCrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacrx_cr::W`](W) writer structure"]
impl crate::Writable for DmacrxCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACRxCR to value 0"]
impl crate::Resettable for DmacrxCrSpec {}
