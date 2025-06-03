#[doc = "Register `FDCAN_TXESC` reader"]
pub type R = crate::R<FdcanTxescSpec>;
#[doc = "Register `FDCAN_TXESC` writer"]
pub type W = crate::W<FdcanTxescSpec>;
#[doc = "Field `TBDS` reader - Tx Buffer Data Field Size:"]
pub type TbdsR = crate::FieldReader;
#[doc = "Field `TBDS` writer - Tx Buffer Data Field Size:"]
pub type TbdsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size:"]
    #[inline(always)]
    pub fn tbds(&self) -> TbdsR {
        TbdsR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size:"]
    #[inline(always)]
    pub fn tbds(&mut self) -> TbdsW<FdcanTxescSpec> {
        TbdsW::new(self, 0)
    }
}
#[doc = "FDCAN Tx Buffer Element Size Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txesc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txesc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTxescSpec;
impl crate::RegisterSpec for FdcanTxescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txesc::R`](R) reader structure"]
impl crate::Readable for FdcanTxescSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txesc::W`](W) writer structure"]
impl crate::Writable for FdcanTxescSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TXESC to value 0"]
impl crate::Resettable for FdcanTxescSpec {}
