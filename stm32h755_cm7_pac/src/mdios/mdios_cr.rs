#[doc = "Register `MDIOS_CR` reader"]
pub type R = crate::R<MdiosCrSpec>;
#[doc = "Register `MDIOS_CR` writer"]
pub type W = crate::W<MdiosCrSpec>;
#[doc = "Field `EN` reader - Peripheral enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Peripheral enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRIE` reader - Register write interrupt enable"]
pub type WrieR = crate::BitReader;
#[doc = "Field `WRIE` writer - Register write interrupt enable"]
pub type WrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDIE` reader - Register Read Interrupt Enable"]
pub type RdieR = crate::BitReader;
#[doc = "Field `RDIE` writer - Register Read Interrupt Enable"]
pub type RdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub type EieR = crate::BitReader;
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub type EieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPC` reader - Disable Preamble Check"]
pub type DpcR = crate::BitReader;
#[doc = "Field `DPC` writer - Disable Preamble Check"]
pub type DpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_ADDRESS` reader - Slaves's address"]
pub type PortAddressR = crate::FieldReader;
#[doc = "Field `PORT_ADDRESS` writer - Slaves's address"]
pub type PortAddressW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Register write interrupt enable"]
    #[inline(always)]
    pub fn wrie(&self) -> WrieR {
        WrieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Register Read Interrupt Enable"]
    #[inline(always)]
    pub fn rdie(&self) -> RdieR {
        RdieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EieR {
        EieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Disable Preamble Check"]
    #[inline(always)]
    pub fn dpc(&self) -> DpcR {
        DpcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Slaves's address"]
    #[inline(always)]
    pub fn port_address(&self) -> PortAddressR {
        PortAddressR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<MdiosCrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Register write interrupt enable"]
    #[inline(always)]
    pub fn wrie(&mut self) -> WrieW<MdiosCrSpec> {
        WrieW::new(self, 1)
    }
    #[doc = "Bit 2 - Register Read Interrupt Enable"]
    #[inline(always)]
    pub fn rdie(&mut self) -> RdieW<MdiosCrSpec> {
        RdieW::new(self, 2)
    }
    #[doc = "Bit 3 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&mut self) -> EieW<MdiosCrSpec> {
        EieW::new(self, 3)
    }
    #[doc = "Bit 7 - Disable Preamble Check"]
    #[inline(always)]
    pub fn dpc(&mut self) -> DpcW<MdiosCrSpec> {
        DpcW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Slaves's address"]
    #[inline(always)]
    pub fn port_address(&mut self) -> PortAddressW<MdiosCrSpec> {
        PortAddressW::new(self, 8)
    }
}
#[doc = "MDIOS configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosCrSpec;
impl crate::RegisterSpec for MdiosCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_cr::R`](R) reader structure"]
impl crate::Readable for MdiosCrSpec {}
#[doc = "`write(|w| ..)` method takes [`mdios_cr::W`](W) writer structure"]
impl crate::Writable for MdiosCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_CR to value 0"]
impl crate::Resettable for MdiosCrSpec {}
