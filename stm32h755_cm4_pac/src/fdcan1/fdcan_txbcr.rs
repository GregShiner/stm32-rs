#[doc = "Register `FDCAN_TXBCR` reader"]
pub type R = crate::R<FdcanTxbcrSpec>;
#[doc = "Register `FDCAN_TXBCR` writer"]
pub type W = crate::W<FdcanTxbcrSpec>;
#[doc = "Field `CR` reader - Cancellation Request"]
pub type CrR = crate::FieldReader<u32>;
#[doc = "Field `CR` writer - Cancellation Request"]
pub type CrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cancellation Request"]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        CrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cancellation Request"]
    #[inline(always)]
    pub fn cr(&mut self) -> CrW<FdcanTxbcrSpec> {
        CrW::new(self, 0)
    }
}
#[doc = "FDCAN Tx Buffer Cancellation Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTxbcrSpec;
impl crate::RegisterSpec for FdcanTxbcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbcr::R`](R) reader structure"]
impl crate::Readable for FdcanTxbcrSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txbcr::W`](W) writer structure"]
impl crate::Writable for FdcanTxbcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TXBCR to value 0"]
impl crate::Resettable for FdcanTxbcrSpec {}
