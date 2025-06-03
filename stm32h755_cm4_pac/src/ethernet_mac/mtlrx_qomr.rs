#[doc = "Register `MTLRxQOMR` reader"]
pub type R = crate::R<MtlrxQomrSpec>;
#[doc = "Register `MTLRxQOMR` writer"]
pub type W = crate::W<MtlrxQomrSpec>;
#[doc = "Field `RTC` reader - RTC"]
pub type RtcR = crate::FieldReader;
#[doc = "Field `RTC` writer - RTC"]
pub type RtcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FUP` reader - FUP"]
pub type FupR = crate::BitReader;
#[doc = "Field `FUP` writer - FUP"]
pub type FupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEP` reader - FEP"]
pub type FepR = crate::BitReader;
#[doc = "Field `FEP` writer - FEP"]
pub type FepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSF` reader - RSF"]
pub type RsfR = crate::BitReader;
#[doc = "Field `RSF` writer - RSF"]
pub type RsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_TCP_EF` reader - DIS_TCP_EF"]
pub type DisTcpEfR = crate::BitReader;
#[doc = "Field `DIS_TCP_EF` writer - DIS_TCP_EF"]
pub type DisTcpEfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EHFC` reader - EHFC"]
pub type EhfcR = crate::BitReader;
#[doc = "Field `EHFC` writer - EHFC"]
pub type EhfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFA` reader - RFA"]
pub type RfaR = crate::FieldReader;
#[doc = "Field `RFA` writer - RFA"]
pub type RfaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RFD` reader - RFD"]
pub type RfdR = crate::FieldReader;
#[doc = "Field `RFD` writer - RFD"]
pub type RfdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RQS` reader - RQS"]
pub type RqsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - FUP"]
    #[inline(always)]
    pub fn fup(&self) -> FupR {
        FupR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FEP"]
    #[inline(always)]
    pub fn fep(&self) -> FepR {
        FepR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RSF"]
    #[inline(always)]
    pub fn rsf(&self) -> RsfR {
        RsfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DIS_TCP_EF"]
    #[inline(always)]
    pub fn dis_tcp_ef(&self) -> DisTcpEfR {
        DisTcpEfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EHFC"]
    #[inline(always)]
    pub fn ehfc(&self) -> EhfcR {
        EhfcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - RFA"]
    #[inline(always)]
    pub fn rfa(&self) -> RfaR {
        RfaR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 14:16 - RFD"]
    #[inline(always)]
    pub fn rfd(&self) -> RfdR {
        RfdR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 20:22 - RQS"]
    #[inline(always)]
    pub fn rqs(&self) -> RqsR {
        RqsR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RTC"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<MtlrxQomrSpec> {
        RtcW::new(self, 0)
    }
    #[doc = "Bit 3 - FUP"]
    #[inline(always)]
    pub fn fup(&mut self) -> FupW<MtlrxQomrSpec> {
        FupW::new(self, 3)
    }
    #[doc = "Bit 4 - FEP"]
    #[inline(always)]
    pub fn fep(&mut self) -> FepW<MtlrxQomrSpec> {
        FepW::new(self, 4)
    }
    #[doc = "Bit 5 - RSF"]
    #[inline(always)]
    pub fn rsf(&mut self) -> RsfW<MtlrxQomrSpec> {
        RsfW::new(self, 5)
    }
    #[doc = "Bit 6 - DIS_TCP_EF"]
    #[inline(always)]
    pub fn dis_tcp_ef(&mut self) -> DisTcpEfW<MtlrxQomrSpec> {
        DisTcpEfW::new(self, 6)
    }
    #[doc = "Bit 7 - EHFC"]
    #[inline(always)]
    pub fn ehfc(&mut self) -> EhfcW<MtlrxQomrSpec> {
        EhfcW::new(self, 7)
    }
    #[doc = "Bits 8:10 - RFA"]
    #[inline(always)]
    pub fn rfa(&mut self) -> RfaW<MtlrxQomrSpec> {
        RfaW::new(self, 8)
    }
    #[doc = "Bits 14:16 - RFD"]
    #[inline(always)]
    pub fn rfd(&mut self) -> RfdW<MtlrxQomrSpec> {
        RfdW::new(self, 14)
    }
}
#[doc = "Rx queue operating mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtlrx_qomr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrx_qomr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtlrxQomrSpec;
impl crate::RegisterSpec for MtlrxQomrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtlrx_qomr::R`](R) reader structure"]
impl crate::Readable for MtlrxQomrSpec {}
#[doc = "`write(|w| ..)` method takes [`mtlrx_qomr::W`](W) writer structure"]
impl crate::Writable for MtlrxQomrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTLRxQOMR to value 0x0070_0000"]
impl crate::Resettable for MtlrxQomrSpec {
    const RESET_VALUE: u32 = 0x0070_0000;
}
