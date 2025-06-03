#[doc = "Register `CR3` reader"]
pub type R = crate::R<Cr3Spec>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<Cr3Spec>;
#[doc = "Field `BYPASS` reader - Power management unit bypass"]
pub type BypassR = crate::BitReader;
#[doc = "Field `BYPASS` writer - Power management unit bypass"]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDOEN` reader - Low drop-out regulator enable"]
pub type LdoenR = crate::BitReader;
#[doc = "Field `LDOEN` writer - Low drop-out regulator enable"]
pub type LdoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDEN` reader - SD converter Enable"]
pub type SdenR = crate::BitReader;
#[doc = "Field `SDEN` writer - SD converter Enable"]
pub type SdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBE` reader - VBAT charging enable"]
pub type VbeR = crate::BitReader;
#[doc = "Field `VBE` writer - VBAT charging enable"]
pub type VbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBRS` reader - VBAT charging resistor selection"]
pub type VbrsR = crate::BitReader;
#[doc = "Field `VBRS` writer - VBAT charging resistor selection"]
pub type VbrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB33DEN` writer - VDD33USB voltage level detector enable."]
pub type Usb33denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBREGEN` reader - USB regulator enable."]
pub type UsbregenR = crate::BitReader;
#[doc = "Field `USBREGEN` writer - USB regulator enable."]
pub type UsbregenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB33RDY` reader - USB supply ready."]
pub type Usb33rdyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Power management unit bypass"]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low drop-out regulator enable"]
    #[inline(always)]
    pub fn ldoen(&self) -> LdoenR {
        LdoenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD converter Enable"]
    #[inline(always)]
    pub fn sden(&self) -> SdenR {
        SdenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - VBAT charging enable"]
    #[inline(always)]
    pub fn vbe(&self) -> VbeR {
        VbeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VBAT charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&self) -> VbrsR {
        VbrsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 25 - USB regulator enable."]
    #[inline(always)]
    pub fn usbregen(&self) -> UsbregenR {
        UsbregenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USB supply ready."]
    #[inline(always)]
    pub fn usb33rdy(&self) -> Usb33rdyR {
        Usb33rdyR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power management unit bypass"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BypassW<Cr3Spec> {
        BypassW::new(self, 0)
    }
    #[doc = "Bit 1 - Low drop-out regulator enable"]
    #[inline(always)]
    pub fn ldoen(&mut self) -> LdoenW<Cr3Spec> {
        LdoenW::new(self, 1)
    }
    #[doc = "Bit 2 - SD converter Enable"]
    #[inline(always)]
    pub fn sden(&mut self) -> SdenW<Cr3Spec> {
        SdenW::new(self, 2)
    }
    #[doc = "Bit 8 - VBAT charging enable"]
    #[inline(always)]
    pub fn vbe(&mut self) -> VbeW<Cr3Spec> {
        VbeW::new(self, 8)
    }
    #[doc = "Bit 9 - VBAT charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&mut self) -> VbrsW<Cr3Spec> {
        VbrsW::new(self, 9)
    }
    #[doc = "Bit 24 - VDD33USB voltage level detector enable."]
    #[inline(always)]
    pub fn usb33den(&mut self) -> Usb33denW<Cr3Spec> {
        Usb33denW::new(self, 24)
    }
    #[doc = "Bit 25 - USB regulator enable."]
    #[inline(always)]
    pub fn usbregen(&mut self) -> UsbregenW<Cr3Spec> {
        UsbregenW::new(self, 25)
    }
}
#[doc = "Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value.\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr3Spec;
impl crate::RegisterSpec for Cr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for Cr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for Cr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR3 to value 0x06"]
impl crate::Resettable for Cr3Spec {
    const RESET_VALUE: u32 = 0x06;
}
