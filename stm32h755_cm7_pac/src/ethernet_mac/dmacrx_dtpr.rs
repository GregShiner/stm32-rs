#[doc = "Register `DMACRxDTPR` reader"]
pub type R = crate::R<DmacrxDtprSpec>;
#[doc = "Register `DMACRxDTPR` writer"]
pub type W = crate::W<DmacrxDtprSpec>;
#[doc = "Field `RDT` reader - Receive Descriptor Tail Pointer"]
pub type RdtR = crate::FieldReader<u32>;
#[doc = "Field `RDT` writer - Receive Descriptor Tail Pointer"]
pub type RdtW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Receive Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn rdt(&self) -> RdtR {
        RdtR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Receive Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn rdt(&mut self) -> RdtW<DmacrxDtprSpec> {
        RdtW::new(self, 2)
    }
}
#[doc = "Channel Rx descriptor tail pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacrx_dtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrx_dtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacrxDtprSpec;
impl crate::RegisterSpec for DmacrxDtprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrx_dtpr::R`](R) reader structure"]
impl crate::Readable for DmacrxDtprSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacrx_dtpr::W`](W) writer structure"]
impl crate::Writable for DmacrxDtprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACRxDTPR to value 0"]
impl crate::Resettable for DmacrxDtprSpec {}
