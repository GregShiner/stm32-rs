#[doc = "Register `DMACRxDLAR` reader"]
pub type R = crate::R<DmacrxDlarSpec>;
#[doc = "Register `DMACRxDLAR` writer"]
pub type W = crate::W<DmacrxDlarSpec>;
#[doc = "Field `RDESLA` reader - Start of Receive List"]
pub type RdeslaR = crate::FieldReader<u32>;
#[doc = "Field `RDESLA` writer - Start of Receive List"]
pub type RdeslaW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Start of Receive List"]
    #[inline(always)]
    pub fn rdesla(&self) -> RdeslaR {
        RdeslaR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of Receive List"]
    #[inline(always)]
    pub fn rdesla(&mut self) -> RdeslaW<DmacrxDlarSpec> {
        RdeslaW::new(self, 2)
    }
}
#[doc = "Channel Rx descriptor list address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacrx_dlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrx_dlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacrxDlarSpec;
impl crate::RegisterSpec for DmacrxDlarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrx_dlar::R`](R) reader structure"]
impl crate::Readable for DmacrxDlarSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacrx_dlar::W`](W) writer structure"]
impl crate::Writable for DmacrxDlarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACRxDLAR to value 0"]
impl crate::Resettable for DmacrxDlarSpec {}
