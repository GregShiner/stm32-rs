#[doc = "Register `OTG_HS_DIEPMSK` reader"]
pub type R = crate::R<OtgHsDiepmskSpec>;
#[doc = "Register `OTG_HS_DIEPMSK` writer"]
pub type W = crate::W<OtgHsDiepmskSpec>;
#[doc = "Field `XFRCM` reader - Transfer completed interrupt mask"]
pub type XfrcmR = crate::BitReader;
#[doc = "Field `XFRCM` writer - Transfer completed interrupt mask"]
pub type XfrcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDM` reader - Endpoint disabled interrupt mask"]
pub type EpdmR = crate::BitReader;
#[doc = "Field `EPDM` writer - Endpoint disabled interrupt mask"]
pub type EpdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOM` reader - Timeout condition mask (nonisochronous endpoints)"]
pub type TomR = crate::BitReader;
#[doc = "Field `TOM` writer - Timeout condition mask (nonisochronous endpoints)"]
pub type TomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITTXFEMSK` reader - IN token received when TxFIFO empty mask"]
pub type IttxfemskR = crate::BitReader;
#[doc = "Field `ITTXFEMSK` writer - IN token received when TxFIFO empty mask"]
pub type IttxfemskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNMM` reader - IN token received with EP mismatch mask"]
pub type InepnmmR = crate::BitReader;
#[doc = "Field `INEPNMM` writer - IN token received with EP mismatch mask"]
pub type InepnmmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNEM` reader - IN endpoint NAK effective mask"]
pub type InepnemR = crate::BitReader;
#[doc = "Field `INEPNEM` writer - IN endpoint NAK effective mask"]
pub type InepnemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFURM` reader - FIFO underrun mask"]
pub type TxfurmR = crate::BitReader;
#[doc = "Field `TXFURM` writer - FIFO underrun mask"]
pub type TxfurmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIM` reader - BNA interrupt mask"]
pub type BimR = crate::BitReader;
#[doc = "Field `BIM` writer - BNA interrupt mask"]
pub type BimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XfrcmR {
        XfrcmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn epdm(&self) -> EpdmR {
        EpdmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout condition mask (nonisochronous endpoints)"]
    #[inline(always)]
    pub fn tom(&self) -> TomR {
        TomR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    pub fn ittxfemsk(&self) -> IttxfemskR {
        IttxfemskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    pub fn inepnmm(&self) -> InepnmmR {
        InepnmmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    pub fn inepnem(&self) -> InepnemR {
        InepnemR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - FIFO underrun mask"]
    #[inline(always)]
    pub fn txfurm(&self) -> TxfurmR {
        TxfurmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn bim(&self) -> BimR {
        BimR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XfrcmW<OtgHsDiepmskSpec> {
        XfrcmW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn epdm(&mut self) -> EpdmW<OtgHsDiepmskSpec> {
        EpdmW::new(self, 1)
    }
    #[doc = "Bit 3 - Timeout condition mask (nonisochronous endpoints)"]
    #[inline(always)]
    pub fn tom(&mut self) -> TomW<OtgHsDiepmskSpec> {
        TomW::new(self, 3)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    pub fn ittxfemsk(&mut self) -> IttxfemskW<OtgHsDiepmskSpec> {
        IttxfemskW::new(self, 4)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    pub fn inepnmm(&mut self) -> InepnmmW<OtgHsDiepmskSpec> {
        InepnmmW::new(self, 5)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    pub fn inepnem(&mut self) -> InepnemW<OtgHsDiepmskSpec> {
        InepnemW::new(self, 6)
    }
    #[doc = "Bit 8 - FIFO underrun mask"]
    #[inline(always)]
    pub fn txfurm(&mut self) -> TxfurmW<OtgHsDiepmskSpec> {
        TxfurmW::new(self, 8)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn bim(&mut self) -> BimW<OtgHsDiepmskSpec> {
        BimW::new(self, 9)
    }
}
#[doc = "OTG_HS device IN endpoint common interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_diepmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsDiepmskSpec;
impl crate::RegisterSpec for OtgHsDiepmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_diepmsk::R`](R) reader structure"]
impl crate::Readable for OtgHsDiepmskSpec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_diepmsk::W`](W) writer structure"]
impl crate::Writable for OtgHsDiepmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_DIEPMSK to value 0"]
impl crate::Resettable for OtgHsDiepmskSpec {}
