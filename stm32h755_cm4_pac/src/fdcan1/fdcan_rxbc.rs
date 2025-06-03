#[doc = "Register `FDCAN_RXBC` reader"]
pub type R = crate::R<FdcanRxbcSpec>;
#[doc = "Register `FDCAN_RXBC` writer"]
pub type W = crate::W<FdcanRxbcSpec>;
#[doc = "Field `RBSA` reader - Rx Buffer Start Address"]
pub type RbsaR = crate::FieldReader<u16>;
#[doc = "Field `RBSA` writer - Rx Buffer Start Address"]
pub type RbsaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 2:15 - Rx Buffer Start Address"]
    #[inline(always)]
    pub fn rbsa(&self) -> RbsaR {
        RbsaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:15 - Rx Buffer Start Address"]
    #[inline(always)]
    pub fn rbsa(&mut self) -> RbsaW<FdcanRxbcSpec> {
        RbsaW::new(self, 2)
    }
}
#[doc = "FDCAN Rx Buffer Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rxbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanRxbcSpec;
impl crate::RegisterSpec for FdcanRxbcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxbc::R`](R) reader structure"]
impl crate::Readable for FdcanRxbcSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rxbc::W`](W) writer structure"]
impl crate::Writable for FdcanRxbcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_RXBC to value 0"]
impl crate::Resettable for FdcanRxbcSpec {}
