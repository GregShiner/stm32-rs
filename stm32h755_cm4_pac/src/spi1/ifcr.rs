#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IfcrSpec>;
#[doc = "Field `EOTC` writer - End Of Transfer flag clear"]
pub type EotcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTFC` writer - Transmission Transfer Filled flag clear"]
pub type TxtfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDRC` writer - Underrun flag clear"]
pub type UdrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRC` writer - Overrun flag clear"]
pub type OvrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEC` writer - CRC Error flag clear"]
pub type CrcecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIFREC` writer - TI frame format error flag clear"]
pub type TifrecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODFC` writer - Mode Fault flag clear"]
pub type ModfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSERFC` writer - TSERFC flag clear"]
pub type TserfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPC` writer - SUSPend flag clear"]
pub type SuspcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 3 - End Of Transfer flag clear"]
    #[inline(always)]
    pub fn eotc(&mut self) -> EotcW<IfcrSpec> {
        EotcW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmission Transfer Filled flag clear"]
    #[inline(always)]
    pub fn txtfc(&mut self) -> TxtfcW<IfcrSpec> {
        TxtfcW::new(self, 4)
    }
    #[doc = "Bit 5 - Underrun flag clear"]
    #[inline(always)]
    pub fn udrc(&mut self) -> UdrcW<IfcrSpec> {
        UdrcW::new(self, 5)
    }
    #[doc = "Bit 6 - Overrun flag clear"]
    #[inline(always)]
    pub fn ovrc(&mut self) -> OvrcW<IfcrSpec> {
        OvrcW::new(self, 6)
    }
    #[doc = "Bit 7 - CRC Error flag clear"]
    #[inline(always)]
    pub fn crcec(&mut self) -> CrcecW<IfcrSpec> {
        CrcecW::new(self, 7)
    }
    #[doc = "Bit 8 - TI frame format error flag clear"]
    #[inline(always)]
    pub fn tifrec(&mut self) -> TifrecW<IfcrSpec> {
        TifrecW::new(self, 8)
    }
    #[doc = "Bit 9 - Mode Fault flag clear"]
    #[inline(always)]
    pub fn modfc(&mut self) -> ModfcW<IfcrSpec> {
        ModfcW::new(self, 9)
    }
    #[doc = "Bit 10 - TSERFC flag clear"]
    #[inline(always)]
    pub fn tserfc(&mut self) -> TserfcW<IfcrSpec> {
        TserfcW::new(self, 10)
    }
    #[doc = "Bit 11 - SUSPend flag clear"]
    #[inline(always)]
    pub fn suspc(&mut self) -> SuspcW<IfcrSpec> {
        SuspcW::new(self, 11)
    }
}
#[doc = "Interrupt/Status Flags Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcrSpec;
impl crate::RegisterSpec for IfcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifcr::W`](W) writer structure"]
impl crate::Writable for IfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IfcrSpec {}
