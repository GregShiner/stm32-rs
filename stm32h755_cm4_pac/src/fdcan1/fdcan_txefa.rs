#[doc = "Register `FDCAN_TXEFA` reader"]
pub type R = crate::R<FdcanTxefaSpec>;
#[doc = "Register `FDCAN_TXEFA` writer"]
pub type W = crate::W<FdcanTxefaSpec>;
#[doc = "Field `EFAI` reader - Event FIFO Acknowledge Index"]
pub type EfaiR = crate::FieldReader;
#[doc = "Field `EFAI` writer - Event FIFO Acknowledge Index"]
pub type EfaiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Event FIFO Acknowledge Index"]
    #[inline(always)]
    pub fn efai(&self) -> EfaiR {
        EfaiR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Event FIFO Acknowledge Index"]
    #[inline(always)]
    pub fn efai(&mut self) -> EfaiW<FdcanTxefaSpec> {
        EfaiW::new(self, 0)
    }
}
#[doc = "FDCAN Tx Event FIFO Acknowledge Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txefa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txefa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTxefaSpec;
impl crate::RegisterSpec for FdcanTxefaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txefa::R`](R) reader structure"]
impl crate::Readable for FdcanTxefaSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txefa::W`](W) writer structure"]
impl crate::Writable for FdcanTxefaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TXEFA to value 0"]
impl crate::Resettable for FdcanTxefaSpec {}
