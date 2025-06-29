#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `SPDIFRXEN` reader - Peripheral Block Enable"]
pub type SpdifrxenR = crate::FieldReader;
#[doc = "Field `SPDIFRXEN` writer - Peripheral Block Enable"]
pub type SpdifrxenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXDMAEN` reader - Receiver DMA ENable for data flow"]
pub type RxdmaenR = crate::BitReader;
#[doc = "Field `RXDMAEN` writer - Receiver DMA ENable for data flow"]
pub type RxdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTEO` reader - STerEO Mode"]
pub type RxsteoR = crate::BitReader;
#[doc = "Field `RXSTEO` writer - STerEO Mode"]
pub type RxsteoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRFMT` reader - RX Data format"]
pub type DrfmtR = crate::FieldReader;
#[doc = "Field `DRFMT` writer - RX Data format"]
pub type DrfmtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PMSK` reader - Mask Parity error bit"]
pub type PmskR = crate::BitReader;
#[doc = "Field `PMSK` writer - Mask Parity error bit"]
pub type PmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMSK` reader - Mask of Validity bit"]
pub type VmskR = crate::BitReader;
#[doc = "Field `VMSK` writer - Mask of Validity bit"]
pub type VmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUMSK` reader - Mask of channel status and user bits"]
pub type CumskR = crate::BitReader;
#[doc = "Field `CUMSK` writer - Mask of channel status and user bits"]
pub type CumskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTMSK` reader - Mask of Preamble Type bits"]
pub type PtmskR = crate::BitReader;
#[doc = "Field `PTMSK` writer - Mask of Preamble Type bits"]
pub type PtmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBDMAEN` reader - Control Buffer DMA ENable for control flow"]
pub type CbdmaenR = crate::BitReader;
#[doc = "Field `CBDMAEN` writer - Control Buffer DMA ENable for control flow"]
pub type CbdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL` reader - Channel Selection"]
pub type ChselR = crate::BitReader;
#[doc = "Field `CHSEL` writer - Channel Selection"]
pub type ChselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBTR` reader - Maximum allowed re-tries during synchronization phase"]
pub type NbtrR = crate::FieldReader;
#[doc = "Field `NBTR` writer - Maximum allowed re-tries during synchronization phase"]
pub type NbtrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WFA` reader - Wait For Activity"]
pub type WfaR = crate::BitReader;
#[doc = "Field `WFA` writer - Wait For Activity"]
pub type WfaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSEL` reader - input selection"]
pub type InselR = crate::FieldReader;
#[doc = "Field `INSEL` writer - input selection"]
pub type InselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CKSEN` reader - Symbol Clock Enable"]
pub type CksenR = crate::BitReader;
#[doc = "Field `CKSEN` writer - Symbol Clock Enable"]
pub type CksenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKSBKPEN` reader - Backup Symbol Clock Enable"]
pub type CksbkpenR = crate::BitReader;
#[doc = "Field `CKSBKPEN` writer - Backup Symbol Clock Enable"]
pub type CksbkpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Peripheral Block Enable"]
    #[inline(always)]
    pub fn spdifrxen(&self) -> SpdifrxenR {
        SpdifrxenR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Receiver DMA ENable for data flow"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RxdmaenR {
        RxdmaenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STerEO Mode"]
    #[inline(always)]
    pub fn rxsteo(&self) -> RxsteoR {
        RxsteoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - RX Data format"]
    #[inline(always)]
    pub fn drfmt(&self) -> DrfmtR {
        DrfmtR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Mask Parity error bit"]
    #[inline(always)]
    pub fn pmsk(&self) -> PmskR {
        PmskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask of Validity bit"]
    #[inline(always)]
    pub fn vmsk(&self) -> VmskR {
        VmskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask of channel status and user bits"]
    #[inline(always)]
    pub fn cumsk(&self) -> CumskR {
        CumskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask of Preamble Type bits"]
    #[inline(always)]
    pub fn ptmsk(&self) -> PtmskR {
        PtmskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Control Buffer DMA ENable for control flow"]
    #[inline(always)]
    pub fn cbdmaen(&self) -> CbdmaenR {
        CbdmaenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel Selection"]
    #[inline(always)]
    pub fn chsel(&self) -> ChselR {
        ChselR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Maximum allowed re-tries during synchronization phase"]
    #[inline(always)]
    pub fn nbtr(&self) -> NbtrR {
        NbtrR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Wait For Activity"]
    #[inline(always)]
    pub fn wfa(&self) -> WfaR {
        WfaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:18 - input selection"]
    #[inline(always)]
    pub fn insel(&self) -> InselR {
        InselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Symbol Clock Enable"]
    #[inline(always)]
    pub fn cksen(&self) -> CksenR {
        CksenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Backup Symbol Clock Enable"]
    #[inline(always)]
    pub fn cksbkpen(&self) -> CksbkpenR {
        CksbkpenR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Peripheral Block Enable"]
    #[inline(always)]
    pub fn spdifrxen(&mut self) -> SpdifrxenW<CrSpec> {
        SpdifrxenW::new(self, 0)
    }
    #[doc = "Bit 2 - Receiver DMA ENable for data flow"]
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RxdmaenW<CrSpec> {
        RxdmaenW::new(self, 2)
    }
    #[doc = "Bit 3 - STerEO Mode"]
    #[inline(always)]
    pub fn rxsteo(&mut self) -> RxsteoW<CrSpec> {
        RxsteoW::new(self, 3)
    }
    #[doc = "Bits 4:5 - RX Data format"]
    #[inline(always)]
    pub fn drfmt(&mut self) -> DrfmtW<CrSpec> {
        DrfmtW::new(self, 4)
    }
    #[doc = "Bit 6 - Mask Parity error bit"]
    #[inline(always)]
    pub fn pmsk(&mut self) -> PmskW<CrSpec> {
        PmskW::new(self, 6)
    }
    #[doc = "Bit 7 - Mask of Validity bit"]
    #[inline(always)]
    pub fn vmsk(&mut self) -> VmskW<CrSpec> {
        VmskW::new(self, 7)
    }
    #[doc = "Bit 8 - Mask of channel status and user bits"]
    #[inline(always)]
    pub fn cumsk(&mut self) -> CumskW<CrSpec> {
        CumskW::new(self, 8)
    }
    #[doc = "Bit 9 - Mask of Preamble Type bits"]
    #[inline(always)]
    pub fn ptmsk(&mut self) -> PtmskW<CrSpec> {
        PtmskW::new(self, 9)
    }
    #[doc = "Bit 10 - Control Buffer DMA ENable for control flow"]
    #[inline(always)]
    pub fn cbdmaen(&mut self) -> CbdmaenW<CrSpec> {
        CbdmaenW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel Selection"]
    #[inline(always)]
    pub fn chsel(&mut self) -> ChselW<CrSpec> {
        ChselW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Maximum allowed re-tries during synchronization phase"]
    #[inline(always)]
    pub fn nbtr(&mut self) -> NbtrW<CrSpec> {
        NbtrW::new(self, 12)
    }
    #[doc = "Bit 14 - Wait For Activity"]
    #[inline(always)]
    pub fn wfa(&mut self) -> WfaW<CrSpec> {
        WfaW::new(self, 14)
    }
    #[doc = "Bits 16:18 - input selection"]
    #[inline(always)]
    pub fn insel(&mut self) -> InselW<CrSpec> {
        InselW::new(self, 16)
    }
    #[doc = "Bit 20 - Symbol Clock Enable"]
    #[inline(always)]
    pub fn cksen(&mut self) -> CksenW<CrSpec> {
        CksenW::new(self, 20)
    }
    #[doc = "Bit 21 - Backup Symbol Clock Enable"]
    #[inline(always)]
    pub fn cksbkpen(&mut self) -> CksbkpenW<CrSpec> {
        CksbkpenW::new(self, 21)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
