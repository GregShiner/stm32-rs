#[doc = "Register `DSI_VPCR` reader"]
pub type R = crate::R<DsiVpcrSpec>;
#[doc = "Register `DSI_VPCR` writer"]
pub type W = crate::W<DsiVpcrSpec>;
#[doc = "Field `VPSIZE` reader - VPSIZE"]
pub type VpsizeR = crate::FieldReader<u16>;
#[doc = "Field `VPSIZE` writer - VPSIZE"]
pub type VpsizeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - VPSIZE"]
    #[inline(always)]
    pub fn vpsize(&self) -> VpsizeR {
        VpsizeR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - VPSIZE"]
    #[inline(always)]
    pub fn vpsize(&mut self) -> VpsizeW<DsiVpcrSpec> {
        VpsizeW::new(self, 0)
    }
}
#[doc = "DSI Host video packet configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVpcrSpec;
impl crate::RegisterSpec for DsiVpcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vpcr::R`](R) reader structure"]
impl crate::Readable for DsiVpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_vpcr::W`](W) writer structure"]
impl crate::Writable for DsiVpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_VPCR to value 0"]
impl crate::Resettable for DsiVpcrSpec {}
