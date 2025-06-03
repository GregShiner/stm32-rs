#[doc = "Register `DMAMR` reader"]
pub type R = crate::R<DmamrSpec>;
#[doc = "Register `DMAMR` writer"]
pub type W = crate::W<DmamrSpec>;
#[doc = "Field `SWR` reader - Software Reset"]
pub type SwrR = crate::BitReader;
#[doc = "Field `SWR` writer - Software Reset"]
pub type SwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DA` reader - DMA Tx or Rx Arbitration Scheme"]
pub type DaR = crate::BitReader;
#[doc = "Field `TXPR` reader - Transmit priority"]
pub type TxprR = crate::BitReader;
#[doc = "Field `PR` reader - Priority ratio"]
pub type PrR = crate::FieldReader;
#[doc = "Field `INTM` reader - Interrupt Mode"]
pub type IntmR = crate::BitReader;
#[doc = "Field `INTM` writer - Interrupt Mode"]
pub type IntmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&self) -> SwrR {
        SwrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Tx or Rx Arbitration Scheme"]
    #[inline(always)]
    pub fn da(&self) -> DaR {
        DaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit priority"]
    #[inline(always)]
    pub fn txpr(&self) -> TxprR {
        TxprR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Priority ratio"]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Interrupt Mode"]
    #[inline(always)]
    pub fn intm(&self) -> IntmR {
        IntmR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&mut self) -> SwrW<DmamrSpec> {
        SwrW::new(self, 0)
    }
    #[doc = "Bit 16 - Interrupt Mode"]
    #[inline(always)]
    pub fn intm(&mut self) -> IntmW<DmamrSpec> {
        IntmW::new(self, 16)
    }
}
#[doc = "DMA mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmamr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmamrSpec;
impl crate::RegisterSpec for DmamrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamr::R`](R) reader structure"]
impl crate::Readable for DmamrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmamr::W`](W) writer structure"]
impl crate::Writable for DmamrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMAMR to value 0"]
impl crate::Resettable for DmamrSpec {}
