#[doc = "Register `DMACTxDTPR` reader"]
pub type R = crate::R<DmactxDtprSpec>;
#[doc = "Register `DMACTxDTPR` writer"]
pub type W = crate::W<DmactxDtprSpec>;
#[doc = "Field `TDT` reader - Transmit Descriptor Tail Pointer"]
pub type TdtR = crate::FieldReader<u32>;
#[doc = "Field `TDT` writer - Transmit Descriptor Tail Pointer"]
pub type TdtW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Transmit Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn tdt(&self) -> TdtR {
        TdtR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Transmit Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn tdt(&mut self) -> TdtW<DmactxDtprSpec> {
        TdtW::new(self, 2)
    }
}
#[doc = "Channel Tx descriptor tail pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactx_dtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactx_dtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmactxDtprSpec;
impl crate::RegisterSpec for DmactxDtprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactx_dtpr::R`](R) reader structure"]
impl crate::Readable for DmactxDtprSpec {}
#[doc = "`write(|w| ..)` method takes [`dmactx_dtpr::W`](W) writer structure"]
impl crate::Writable for DmactxDtprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACTxDTPR to value 0"]
impl crate::Resettable for DmactxDtprSpec {}
