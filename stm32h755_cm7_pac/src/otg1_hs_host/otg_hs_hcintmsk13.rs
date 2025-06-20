#[doc = "Register `OTG_HS_HCINTMSK13` reader"]
pub type R = crate::R<OtgHsHcintmsk13Spec>;
#[doc = "Register `OTG_HS_HCINTMSK13` writer"]
pub type W = crate::W<OtgHsHcintmsk13Spec>;
#[doc = "Field `XFRCM` reader - Transfer completed mask"]
pub type XfrcmR = crate::BitReader;
#[doc = "Field `XFRCM` writer - Transfer completed mask"]
pub type XfrcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHHM` reader - Channel halted mask"]
pub type ChhmR = crate::BitReader;
#[doc = "Field `CHHM` writer - Channel halted mask"]
pub type ChhmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERR` reader - AHB error"]
pub type AhberrR = crate::BitReader;
#[doc = "Field `AHBERR` writer - AHB error"]
pub type AhberrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLM` reader - STALLM response received interrupt mask"]
pub type StallmR = crate::BitReader;
#[doc = "Field `STALLM` writer - STALLM response received interrupt mask"]
pub type StallmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKM` reader - NAK response received interrupt mask"]
pub type NakmR = crate::BitReader;
#[doc = "Field `NAKM` writer - NAK response received interrupt mask"]
pub type NakmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKM` reader - ACK response received/transmitted interrupt mask"]
pub type AckmR = crate::BitReader;
#[doc = "Field `ACKM` writer - ACK response received/transmitted interrupt mask"]
pub type AckmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYET` reader - Response received interrupt"]
pub type NyetR = crate::BitReader;
#[doc = "Field `NYET` writer - Response received interrupt"]
pub type NyetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERRM` reader - Transaction error"]
pub type TxerrmR = crate::BitReader;
#[doc = "Field `TXERRM` writer - Transaction error"]
pub type TxerrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBERRM` reader - Babble error"]
pub type BberrmR = crate::BitReader;
#[doc = "Field `BBERRM` writer - Babble error"]
pub type BberrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMORM` reader - Frame overrun mask"]
pub type FrmormR = crate::BitReader;
#[doc = "Field `FRMORM` writer - Frame overrun mask"]
pub type FrmormW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRM` reader - Data toggle error mask"]
pub type DterrmR = crate::BitReader;
#[doc = "Field `DTERRM` writer - Data toggle error mask"]
pub type DterrmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed mask"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XfrcmR {
        XfrcmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted mask"]
    #[inline(always)]
    pub fn chhm(&self) -> ChhmR {
        ChhmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AhberrR {
        AhberrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALLM response received interrupt mask"]
    #[inline(always)]
    pub fn stallm(&self) -> StallmR {
        StallmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK response received interrupt mask"]
    #[inline(always)]
    pub fn nakm(&self) -> NakmR {
        NakmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt mask"]
    #[inline(always)]
    pub fn ackm(&self) -> AckmR {
        AckmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Response received interrupt"]
    #[inline(always)]
    pub fn nyet(&self) -> NyetR {
        NyetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction error"]
    #[inline(always)]
    pub fn txerrm(&self) -> TxerrmR {
        TxerrmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    pub fn bberrm(&self) -> BberrmR {
        BberrmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame overrun mask"]
    #[inline(always)]
    pub fn frmorm(&self) -> FrmormR {
        FrmormR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error mask"]
    #[inline(always)]
    pub fn dterrm(&self) -> DterrmR {
        DterrmR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed mask"]
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XfrcmW<OtgHsHcintmsk13Spec> {
        XfrcmW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel halted mask"]
    #[inline(always)]
    pub fn chhm(&mut self) -> ChhmW<OtgHsHcintmsk13Spec> {
        ChhmW::new(self, 1)
    }
    #[doc = "Bit 2 - AHB error"]
    #[inline(always)]
    pub fn ahberr(&mut self) -> AhberrW<OtgHsHcintmsk13Spec> {
        AhberrW::new(self, 2)
    }
    #[doc = "Bit 3 - STALLM response received interrupt mask"]
    #[inline(always)]
    pub fn stallm(&mut self) -> StallmW<OtgHsHcintmsk13Spec> {
        StallmW::new(self, 3)
    }
    #[doc = "Bit 4 - NAK response received interrupt mask"]
    #[inline(always)]
    pub fn nakm(&mut self) -> NakmW<OtgHsHcintmsk13Spec> {
        NakmW::new(self, 4)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt mask"]
    #[inline(always)]
    pub fn ackm(&mut self) -> AckmW<OtgHsHcintmsk13Spec> {
        AckmW::new(self, 5)
    }
    #[doc = "Bit 6 - Response received interrupt"]
    #[inline(always)]
    pub fn nyet(&mut self) -> NyetW<OtgHsHcintmsk13Spec> {
        NyetW::new(self, 6)
    }
    #[doc = "Bit 7 - Transaction error"]
    #[inline(always)]
    pub fn txerrm(&mut self) -> TxerrmW<OtgHsHcintmsk13Spec> {
        TxerrmW::new(self, 7)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    pub fn bberrm(&mut self) -> BberrmW<OtgHsHcintmsk13Spec> {
        BberrmW::new(self, 8)
    }
    #[doc = "Bit 9 - Frame overrun mask"]
    #[inline(always)]
    pub fn frmorm(&mut self) -> FrmormW<OtgHsHcintmsk13Spec> {
        FrmormW::new(self, 9)
    }
    #[doc = "Bit 10 - Data toggle error mask"]
    #[inline(always)]
    pub fn dterrm(&mut self) -> DterrmW<OtgHsHcintmsk13Spec> {
        DterrmW::new(self, 10)
    }
}
#[doc = "OTG_HS host channel-13 interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_hcintmsk13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcintmsk13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsHcintmsk13Spec;
impl crate::RegisterSpec for OtgHsHcintmsk13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_hcintmsk13::R`](R) reader structure"]
impl crate::Readable for OtgHsHcintmsk13Spec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_hcintmsk13::W`](W) writer structure"]
impl crate::Writable for OtgHsHcintmsk13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_HCINTMSK13 to value 0"]
impl crate::Resettable for OtgHsHcintmsk13Spec {}
