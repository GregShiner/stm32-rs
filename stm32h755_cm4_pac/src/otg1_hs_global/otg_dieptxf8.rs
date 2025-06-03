#[doc = "Register `OTG_DIEPTXF8` reader"]
pub type R = crate::R<OtgDieptxf8Spec>;
#[doc = "Register `OTG_DIEPTXF8` writer"]
pub type W = crate::W<OtgDieptxf8Spec>;
#[doc = "Field `INEPTXSA` reader - IN endpoint FIFOx transmit RAM start address This field contains the memory start address for IN endpoint transmit FIFOx. The address must be aligned with a 32-bit memory location."]
pub type IneptxsaR = crate::FieldReader<u16>;
#[doc = "Field `INEPTXSA` writer - IN endpoint FIFOx transmit RAM start address This field contains the memory start address for IN endpoint transmit FIFOx. The address must be aligned with a 32-bit memory location."]
pub type IneptxsaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INEPTXFD` reader - IN endpoint Tx FIFO depth This value is in terms of 32-bit words. Minimum value is 16"]
pub type IneptxfdR = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFD` writer - IN endpoint Tx FIFO depth This value is in terms of 32-bit words. Minimum value is 16"]
pub type IneptxfdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address This field contains the memory start address for IN endpoint transmit FIFOx. The address must be aligned with a 32-bit memory location."]
    #[inline(always)]
    pub fn ineptxsa(&self) -> IneptxsaR {
        IneptxsaR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN endpoint Tx FIFO depth This value is in terms of 32-bit words. Minimum value is 16"]
    #[inline(always)]
    pub fn ineptxfd(&self) -> IneptxfdR {
        IneptxfdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address This field contains the memory start address for IN endpoint transmit FIFOx. The address must be aligned with a 32-bit memory location."]
    #[inline(always)]
    pub fn ineptxsa(&mut self) -> IneptxsaW<OtgDieptxf8Spec> {
        IneptxsaW::new(self, 0)
    }
    #[doc = "Bits 16:31 - IN endpoint Tx FIFO depth This value is in terms of 32-bit words. Minimum value is 16"]
    #[inline(always)]
    pub fn ineptxfd(&mut self) -> IneptxfdW<OtgDieptxf8Spec> {
        IneptxfdW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_dieptxf8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_dieptxf8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgDieptxf8Spec;
impl crate::RegisterSpec for OtgDieptxf8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_dieptxf8::R`](R) reader structure"]
impl crate::Readable for OtgDieptxf8Spec {}
#[doc = "`write(|w| ..)` method takes [`otg_dieptxf8::W`](W) writer structure"]
impl crate::Writable for OtgDieptxf8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_DIEPTXF8 to value 0x0200_1200"]
impl crate::Resettable for OtgDieptxf8Spec {
    const RESET_VALUE: u32 = 0x0200_1200;
}
