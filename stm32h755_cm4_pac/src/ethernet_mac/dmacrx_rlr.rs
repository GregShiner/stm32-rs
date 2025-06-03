#[doc = "Register `DMACRxRLR` reader"]
pub type R = crate::R<DmacrxRlrSpec>;
#[doc = "Register `DMACRxRLR` writer"]
pub type W = crate::W<DmacrxRlrSpec>;
#[doc = "Field `RDRL` reader - Receive Descriptor Ring Length"]
pub type RdrlR = crate::FieldReader<u16>;
#[doc = "Field `RDRL` writer - Receive Descriptor Ring Length"]
pub type RdrlW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Receive Descriptor Ring Length"]
    #[inline(always)]
    pub fn rdrl(&self) -> RdrlR {
        RdrlR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Receive Descriptor Ring Length"]
    #[inline(always)]
    pub fn rdrl(&mut self) -> RdrlW<DmacrxRlrSpec> {
        RdrlW::new(self, 0)
    }
}
#[doc = "Channel Rx descriptor ring length register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacrx_rlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrx_rlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacrxRlrSpec;
impl crate::RegisterSpec for DmacrxRlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrx_rlr::R`](R) reader structure"]
impl crate::Readable for DmacrxRlrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacrx_rlr::W`](W) writer structure"]
impl crate::Writable for DmacrxRlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACRxRLR to value 0"]
impl crate::Resettable for DmacrxRlrSpec {}
