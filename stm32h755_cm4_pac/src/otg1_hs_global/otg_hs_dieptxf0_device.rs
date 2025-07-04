#[doc = "Register `OTG_HS_DIEPTXF0_Device` reader"]
pub type R = crate::R<OtgHsDieptxf0DeviceSpec>;
#[doc = "Register `OTG_HS_DIEPTXF0_Device` writer"]
pub type W = crate::W<OtgHsDieptxf0DeviceSpec>;
#[doc = "Field `TX0FSA` reader - Endpoint 0 transmit RAM start address"]
pub type Tx0fsaR = crate::FieldReader<u16>;
#[doc = "Field `TX0FSA` writer - Endpoint 0 transmit RAM start address"]
pub type Tx0fsaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TX0FD` reader - Endpoint 0 TxFIFO depth"]
pub type Tx0fdR = crate::FieldReader<u16>;
#[doc = "Field `TX0FD` writer - Endpoint 0 TxFIFO depth"]
pub type Tx0fdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    pub fn tx0fsa(&self) -> Tx0fsaR {
        Tx0fsaR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    pub fn tx0fd(&self) -> Tx0fdR {
        Tx0fdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    pub fn tx0fsa(&mut self) -> Tx0fsaW<OtgHsDieptxf0DeviceSpec> {
        Tx0fsaW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    pub fn tx0fd(&mut self) -> Tx0fdW<OtgHsDieptxf0DeviceSpec> {
        Tx0fdW::new(self, 16)
    }
}
#[doc = "Endpoint 0 transmit FIFO size (peripheral mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_dieptxf0_device::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dieptxf0_device::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsDieptxf0DeviceSpec;
impl crate::RegisterSpec for OtgHsDieptxf0DeviceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_dieptxf0_device::R`](R) reader structure"]
impl crate::Readable for OtgHsDieptxf0DeviceSpec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_dieptxf0_device::W`](W) writer structure"]
impl crate::Writable for OtgHsDieptxf0DeviceSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_DIEPTXF0_Device to value 0x0200"]
impl crate::Resettable for OtgHsDieptxf0DeviceSpec {
    const RESET_VALUE: u32 = 0x0200;
}
