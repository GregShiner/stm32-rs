#[doc = "Register `FDCAN_TXBCIE` reader"]
pub type R = crate::R<FdcanTxbcieSpec>;
#[doc = "Register `FDCAN_TXBCIE` writer"]
pub type W = crate::W<FdcanTxbcieSpec>;
#[doc = "Field `CF` reader - Cancellation Finished Interrupt Enable"]
pub type CfR = crate::FieldReader<u32>;
#[doc = "Field `CF` writer - Cancellation Finished Interrupt Enable"]
pub type CfW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    pub fn cf(&self) -> CfR {
        CfR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    pub fn cf(&mut self) -> CfW<FdcanTxbcieSpec> {
        CfW::new(self, 0)
    }
}
#[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbcie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbcie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTxbcieSpec;
impl crate::RegisterSpec for FdcanTxbcieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbcie::R`](R) reader structure"]
impl crate::Readable for FdcanTxbcieSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txbcie::W`](W) writer structure"]
impl crate::Writable for FdcanTxbcieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TXBCIE to value 0"]
impl crate::Resettable for FdcanTxbcieSpec {}
