#[doc = "Register `OTG_HS_HCSPLT5` reader"]
pub type R = crate::R<OtgHsHcsplt5Spec>;
#[doc = "Register `OTG_HS_HCSPLT5` writer"]
pub type W = crate::W<OtgHsHcsplt5Spec>;
#[doc = "Field `PRTADDR` reader - Port address"]
pub type PrtaddrR = crate::FieldReader;
#[doc = "Field `PRTADDR` writer - Port address"]
pub type PrtaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HUBADDR` reader - Hub address"]
pub type HubaddrR = crate::FieldReader;
#[doc = "Field `HUBADDR` writer - Hub address"]
pub type HubaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XACTPOS` reader - XACTPOS"]
pub type XactposR = crate::FieldReader;
#[doc = "Field `XACTPOS` writer - XACTPOS"]
pub type XactposW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMPLSPLT` reader - Do complete split"]
pub type ComplspltR = crate::BitReader;
#[doc = "Field `COMPLSPLT` writer - Do complete split"]
pub type ComplspltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLITEN` reader - Split enable"]
pub type SplitenR = crate::BitReader;
#[doc = "Field `SPLITEN` writer - Split enable"]
pub type SplitenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Port address"]
    #[inline(always)]
    pub fn prtaddr(&self) -> PrtaddrR {
        PrtaddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Hub address"]
    #[inline(always)]
    pub fn hubaddr(&self) -> HubaddrR {
        HubaddrR::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:15 - XACTPOS"]
    #[inline(always)]
    pub fn xactpos(&self) -> XactposR {
        XactposR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Do complete split"]
    #[inline(always)]
    pub fn complsplt(&self) -> ComplspltR {
        ComplspltR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Split enable"]
    #[inline(always)]
    pub fn spliten(&self) -> SplitenR {
        SplitenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Port address"]
    #[inline(always)]
    pub fn prtaddr(&mut self) -> PrtaddrW<OtgHsHcsplt5Spec> {
        PrtaddrW::new(self, 0)
    }
    #[doc = "Bits 7:13 - Hub address"]
    #[inline(always)]
    pub fn hubaddr(&mut self) -> HubaddrW<OtgHsHcsplt5Spec> {
        HubaddrW::new(self, 7)
    }
    #[doc = "Bits 14:15 - XACTPOS"]
    #[inline(always)]
    pub fn xactpos(&mut self) -> XactposW<OtgHsHcsplt5Spec> {
        XactposW::new(self, 14)
    }
    #[doc = "Bit 16 - Do complete split"]
    #[inline(always)]
    pub fn complsplt(&mut self) -> ComplspltW<OtgHsHcsplt5Spec> {
        ComplspltW::new(self, 16)
    }
    #[doc = "Bit 31 - Split enable"]
    #[inline(always)]
    pub fn spliten(&mut self) -> SplitenW<OtgHsHcsplt5Spec> {
        SplitenW::new(self, 31)
    }
}
#[doc = "OTG_HS host channel-5 split control register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_hcsplt5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcsplt5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsHcsplt5Spec;
impl crate::RegisterSpec for OtgHsHcsplt5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_hcsplt5::R`](R) reader structure"]
impl crate::Readable for OtgHsHcsplt5Spec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_hcsplt5::W`](W) writer structure"]
impl crate::Writable for OtgHsHcsplt5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_HCSPLT5 to value 0"]
impl crate::Resettable for OtgHsHcsplt5Spec {}
