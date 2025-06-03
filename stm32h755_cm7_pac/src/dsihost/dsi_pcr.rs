#[doc = "Register `DSI_PCR` reader"]
pub type R = crate::R<DsiPcrSpec>;
#[doc = "Register `DSI_PCR` writer"]
pub type W = crate::W<DsiPcrSpec>;
#[doc = "Field `ETTXE` reader - ETTXE"]
pub type EttxeR = crate::BitReader;
#[doc = "Field `ETTXE` writer - ETTXE"]
pub type EttxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETRXE` reader - ETRXE"]
pub type EtrxeR = crate::BitReader;
#[doc = "Field `ETRXE` writer - ETRXE"]
pub type EtrxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTAE` reader - BTAE"]
pub type BtaeR = crate::BitReader;
#[doc = "Field `BTAE` writer - BTAE"]
pub type BtaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCRXE` reader - ECCRXE"]
pub type EccrxeR = crate::BitReader;
#[doc = "Field `ECCRXE` writer - ECCRXE"]
pub type EccrxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCRXE` reader - CRCRXE"]
pub type CrcrxeR = crate::BitReader;
#[doc = "Field `CRCRXE` writer - CRCRXE"]
pub type CrcrxeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ETTXE"]
    #[inline(always)]
    pub fn ettxe(&self) -> EttxeR {
        EttxeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ETRXE"]
    #[inline(always)]
    pub fn etrxe(&self) -> EtrxeR {
        EtrxeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BTAE"]
    #[inline(always)]
    pub fn btae(&self) -> BtaeR {
        BtaeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECCRXE"]
    #[inline(always)]
    pub fn eccrxe(&self) -> EccrxeR {
        EccrxeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRCRXE"]
    #[inline(always)]
    pub fn crcrxe(&self) -> CrcrxeR {
        CrcrxeR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ETTXE"]
    #[inline(always)]
    pub fn ettxe(&mut self) -> EttxeW<DsiPcrSpec> {
        EttxeW::new(self, 0)
    }
    #[doc = "Bit 1 - ETRXE"]
    #[inline(always)]
    pub fn etrxe(&mut self) -> EtrxeW<DsiPcrSpec> {
        EtrxeW::new(self, 1)
    }
    #[doc = "Bit 2 - BTAE"]
    #[inline(always)]
    pub fn btae(&mut self) -> BtaeW<DsiPcrSpec> {
        BtaeW::new(self, 2)
    }
    #[doc = "Bit 3 - ECCRXE"]
    #[inline(always)]
    pub fn eccrxe(&mut self) -> EccrxeW<DsiPcrSpec> {
        EccrxeW::new(self, 3)
    }
    #[doc = "Bit 4 - CRCRXE"]
    #[inline(always)]
    pub fn crcrxe(&mut self) -> CrcrxeW<DsiPcrSpec> {
        CrcrxeW::new(self, 4)
    }
}
#[doc = "DSI Host protocol configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiPcrSpec;
impl crate::RegisterSpec for DsiPcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_pcr::R`](R) reader structure"]
impl crate::Readable for DsiPcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_pcr::W`](W) writer structure"]
impl crate::Writable for DsiPcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_PCR to value 0"]
impl crate::Resettable for DsiPcrSpec {}
