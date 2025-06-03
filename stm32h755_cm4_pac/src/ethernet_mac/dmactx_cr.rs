#[doc = "Register `DMACTxCR` reader"]
pub type R = crate::R<DmactxCrSpec>;
#[doc = "Register `DMACTxCR` writer"]
pub type W = crate::W<DmactxCrSpec>;
#[doc = "Field `ST` reader - Start or Stop Transmission Command"]
pub type StR = crate::BitReader;
#[doc = "Field `ST` writer - Start or Stop Transmission Command"]
pub type StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSF` reader - Operate on Second Packet"]
pub type OsfR = crate::BitReader;
#[doc = "Field `OSF` writer - Operate on Second Packet"]
pub type OsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSE` reader - TCP Segmentation Enabled"]
pub type TseR = crate::BitReader;
#[doc = "Field `TSE` writer - TCP Segmentation Enabled"]
pub type TseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPBL` reader - Transmit Programmable Burst Length"]
pub type TxpblR = crate::FieldReader;
#[doc = "Field `TXPBL` writer - Transmit Programmable Burst Length"]
pub type TxpblW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Start or Stop Transmission Command"]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Operate on Second Packet"]
    #[inline(always)]
    pub fn osf(&self) -> OsfR {
        OsfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 12 - TCP Segmentation Enabled"]
    #[inline(always)]
    pub fn tse(&self) -> TseR {
        TseR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Transmit Programmable Burst Length"]
    #[inline(always)]
    pub fn txpbl(&self) -> TxpblR {
        TxpblR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start or Stop Transmission Command"]
    #[inline(always)]
    pub fn st(&mut self) -> StW<DmactxCrSpec> {
        StW::new(self, 0)
    }
    #[doc = "Bit 4 - Operate on Second Packet"]
    #[inline(always)]
    pub fn osf(&mut self) -> OsfW<DmactxCrSpec> {
        OsfW::new(self, 4)
    }
    #[doc = "Bit 12 - TCP Segmentation Enabled"]
    #[inline(always)]
    pub fn tse(&mut self) -> TseW<DmactxCrSpec> {
        TseW::new(self, 12)
    }
    #[doc = "Bits 16:21 - Transmit Programmable Burst Length"]
    #[inline(always)]
    pub fn txpbl(&mut self) -> TxpblW<DmactxCrSpec> {
        TxpblW::new(self, 16)
    }
}
#[doc = "Channel transmit control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactx_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactx_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmactxCrSpec;
impl crate::RegisterSpec for DmactxCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactx_cr::R`](R) reader structure"]
impl crate::Readable for DmactxCrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmactx_cr::W`](W) writer structure"]
impl crate::Writable for DmactxCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACTxCR to value 0"]
impl crate::Resettable for DmactxCrSpec {}
