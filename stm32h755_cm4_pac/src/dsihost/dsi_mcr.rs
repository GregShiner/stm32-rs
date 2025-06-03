#[doc = "Register `DSI_MCR` reader"]
pub type R = crate::R<DsiMcrSpec>;
#[doc = "Register `DSI_MCR` writer"]
pub type W = crate::W<DsiMcrSpec>;
#[doc = "Field `CMDM` reader - CMDM"]
pub type CmdmR = crate::BitReader;
#[doc = "Field `CMDM` writer - CMDM"]
pub type CmdmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CMDM"]
    #[inline(always)]
    pub fn cmdm(&self) -> CmdmR {
        CmdmR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMDM"]
    #[inline(always)]
    pub fn cmdm(&mut self) -> CmdmW<DsiMcrSpec> {
        CmdmW::new(self, 0)
    }
}
#[doc = "DSI Host mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiMcrSpec;
impl crate::RegisterSpec for DsiMcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_mcr::R`](R) reader structure"]
impl crate::Readable for DsiMcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_mcr::W`](W) writer structure"]
impl crate::Writable for DsiMcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_MCR to value 0x01"]
impl crate::Resettable for DsiMcrSpec {
    const RESET_VALUE: u32 = 0x01;
}
