#[doc = "Register `DMACTxDLAR` reader"]
pub type R = crate::R<DmactxDlarSpec>;
#[doc = "Register `DMACTxDLAR` writer"]
pub type W = crate::W<DmactxDlarSpec>;
#[doc = "Field `TDESLA` reader - Start of Transmit List"]
pub type TdeslaR = crate::FieldReader<u32>;
#[doc = "Field `TDESLA` writer - Start of Transmit List"]
pub type TdeslaW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Start of Transmit List"]
    #[inline(always)]
    pub fn tdesla(&self) -> TdeslaR {
        TdeslaR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of Transmit List"]
    #[inline(always)]
    pub fn tdesla(&mut self) -> TdeslaW<DmactxDlarSpec> {
        TdeslaW::new(self, 2)
    }
}
#[doc = "Channel Tx descriptor list address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactx_dlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactx_dlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmactxDlarSpec;
impl crate::RegisterSpec for DmactxDlarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactx_dlar::R`](R) reader structure"]
impl crate::Readable for DmactxDlarSpec {}
#[doc = "`write(|w| ..)` method takes [`dmactx_dlar::W`](W) writer structure"]
impl crate::Writable for DmactxDlarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACTxDLAR to value 0"]
impl crate::Resettable for DmactxDlarSpec {}
