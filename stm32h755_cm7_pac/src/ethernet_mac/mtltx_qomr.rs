#[doc = "Register `MTLTxQOMR` reader"]
pub type R = crate::R<MtltxQomrSpec>;
#[doc = "Register `MTLTxQOMR` writer"]
pub type W = crate::W<MtltxQomrSpec>;
#[doc = "Field `FTQ` reader - Flush Transmit Queue"]
pub type FtqR = crate::BitReader;
#[doc = "Field `FTQ` writer - Flush Transmit Queue"]
pub type FtqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSF` reader - Transmit Store and Forward"]
pub type TsfR = crate::BitReader;
#[doc = "Field `TSF` writer - Transmit Store and Forward"]
pub type TsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXQEN` reader - Transmit Queue Enable"]
pub type TxqenR = crate::FieldReader;
#[doc = "Field `TTC` reader - Transmit Threshold Control"]
pub type TtcR = crate::FieldReader;
#[doc = "Field `TTC` writer - Transmit Threshold Control"]
pub type TtcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TQS` reader - Transmit Queue Size"]
pub type TqsR = crate::FieldReader<u16>;
#[doc = "Field `TQS` writer - Transmit Queue Size"]
pub type TqsW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 0 - Flush Transmit Queue"]
    #[inline(always)]
    pub fn ftq(&self) -> FtqR {
        FtqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Store and Forward"]
    #[inline(always)]
    pub fn tsf(&self) -> TsfR {
        TsfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Transmit Queue Enable"]
    #[inline(always)]
    pub fn txqen(&self) -> TxqenR {
        TxqenR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Transmit Threshold Control"]
    #[inline(always)]
    pub fn ttc(&self) -> TtcR {
        TtcR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:24 - Transmit Queue Size"]
    #[inline(always)]
    pub fn tqs(&self) -> TqsR {
        TqsR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flush Transmit Queue"]
    #[inline(always)]
    pub fn ftq(&mut self) -> FtqW<MtltxQomrSpec> {
        FtqW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Store and Forward"]
    #[inline(always)]
    pub fn tsf(&mut self) -> TsfW<MtltxQomrSpec> {
        TsfW::new(self, 1)
    }
    #[doc = "Bits 4:6 - Transmit Threshold Control"]
    #[inline(always)]
    pub fn ttc(&mut self) -> TtcW<MtltxQomrSpec> {
        TtcW::new(self, 4)
    }
    #[doc = "Bits 16:24 - Transmit Queue Size"]
    #[inline(always)]
    pub fn tqs(&mut self) -> TqsW<MtltxQomrSpec> {
        TqsW::new(self, 16)
    }
}
#[doc = "Tx queue operating mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtltx_qomr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltx_qomr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtltxQomrSpec;
impl crate::RegisterSpec for MtltxQomrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtltx_qomr::R`](R) reader structure"]
impl crate::Readable for MtltxQomrSpec {}
#[doc = "`write(|w| ..)` method takes [`mtltx_qomr::W`](W) writer structure"]
impl crate::Writable for MtltxQomrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTLTxQOMR to value 0x0007_0008"]
impl crate::Resettable for MtltxQomrSpec {
    const RESET_VALUE: u32 = 0x0007_0008;
}
