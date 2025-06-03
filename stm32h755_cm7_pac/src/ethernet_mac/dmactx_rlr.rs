#[doc = "Register `DMACTxRLR` reader"]
pub type R = crate::R<DmactxRlrSpec>;
#[doc = "Register `DMACTxRLR` writer"]
pub type W = crate::W<DmactxRlrSpec>;
#[doc = "Field `TDRL` reader - Transmit Descriptor Ring Length"]
pub type TdrlR = crate::FieldReader<u16>;
#[doc = "Field `TDRL` writer - Transmit Descriptor Ring Length"]
pub type TdrlW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Transmit Descriptor Ring Length"]
    #[inline(always)]
    pub fn tdrl(&self) -> TdrlR {
        TdrlR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Transmit Descriptor Ring Length"]
    #[inline(always)]
    pub fn tdrl(&mut self) -> TdrlW<DmactxRlrSpec> {
        TdrlW::new(self, 0)
    }
}
#[doc = "Channel Tx descriptor ring length register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactx_rlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactx_rlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmactxRlrSpec;
impl crate::RegisterSpec for DmactxRlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactx_rlr::R`](R) reader structure"]
impl crate::Readable for DmactxRlrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmactx_rlr::W`](W) writer structure"]
impl crate::Writable for DmactxRlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACTxRLR to value 0"]
impl crate::Resettable for DmactxRlrSpec {}
