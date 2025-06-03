#[doc = "Register `RTC_ALRMBSSR` reader"]
pub type R = crate::R<RtcAlrmbssrSpec>;
#[doc = "Register `RTC_ALRMBSSR` writer"]
pub type W = crate::W<RtcAlrmbssrSpec>;
#[doc = "Field `SS` reader - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if Alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared."]
pub type SsR = crate::FieldReader<u16>;
#[doc = "Field `SS` writer - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if Alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared."]
pub type SsW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `MASKSS` reader - Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
pub type MaskssR = crate::FieldReader;
#[doc = "Field `MASKSS` writer - Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
pub type MaskssW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:14 - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if Alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared."]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
    #[inline(always)]
    pub fn maskss(&self) -> MaskssR {
        MaskssR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if Alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared."]
    #[inline(always)]
    pub fn ss(&mut self) -> SsW<RtcAlrmbssrSpec> {
        SsW::new(self, 0)
    }
    #[doc = "Bits 24:27 - Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
    #[inline(always)]
    pub fn maskss(&mut self) -> MaskssW<RtcAlrmbssrSpec> {
        MaskssW::new(self, 24)
    }
}
#[doc = "This register can be written only when ALRBE is reset in RTC_CR register, or in initialization mode.This register is write protected.The write access procedure is described in Section: RTC register write protection.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_alrmbssr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrmbssr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcAlrmbssrSpec;
impl crate::RegisterSpec for RtcAlrmbssrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_alrmbssr::R`](R) reader structure"]
impl crate::Readable for RtcAlrmbssrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_alrmbssr::W`](W) writer structure"]
impl crate::Writable for RtcAlrmbssrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_ALRMBSSR to value 0"]
impl crate::Resettable for RtcAlrmbssrSpec {}
