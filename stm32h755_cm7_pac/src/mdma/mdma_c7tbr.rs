#[doc = "Register `MDMA_C7TBR` reader"]
pub type R = crate::R<MdmaC7tbrSpec>;
#[doc = "Register `MDMA_C7TBR` writer"]
pub type W = crate::W<MdmaC7tbrSpec>;
#[doc = "Field `TSEL` reader - Trigger selection"]
pub type TselR = crate::FieldReader;
#[doc = "Field `TSEL` writer - Trigger selection"]
pub type TselW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SBUS` reader - Source BUS select This bit is protected and can be written only if EN is 0."]
pub type SbusR = crate::BitReader;
#[doc = "Field `SBUS` writer - Source BUS select This bit is protected and can be written only if EN is 0."]
pub type SbusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUS` reader - Destination BUS slect This bit is protected and can be written only if EN is 0."]
pub type DbusR = crate::BitReader;
#[doc = "Field `DBUS` writer - Destination BUS slect This bit is protected and can be written only if EN is 0."]
pub type DbusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Trigger selection"]
    #[inline(always)]
    pub fn tsel(&self) -> TselR {
        TselR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Source BUS select This bit is protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn sbus(&self) -> SbusR {
        SbusR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Destination BUS slect This bit is protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn dbus(&self) -> DbusR {
        DbusR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trigger selection"]
    #[inline(always)]
    pub fn tsel(&mut self) -> TselW<MdmaC7tbrSpec> {
        TselW::new(self, 0)
    }
    #[doc = "Bit 16 - Source BUS select This bit is protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn sbus(&mut self) -> SbusW<MdmaC7tbrSpec> {
        SbusW::new(self, 16)
    }
    #[doc = "Bit 17 - Destination BUS slect This bit is protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn dbus(&mut self) -> DbusW<MdmaC7tbrSpec> {
        DbusW::new(self, 17)
    }
}
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c7tbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7tbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC7tbrSpec;
impl crate::RegisterSpec for MdmaC7tbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c7tbr::R`](R) reader structure"]
impl crate::Readable for MdmaC7tbrSpec {}
#[doc = "`write(|w| ..)` method takes [`mdma_c7tbr::W`](W) writer structure"]
impl crate::Writable for MdmaC7tbrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C7TBR to value 0"]
impl crate::Resettable for MdmaC7tbrSpec {}
