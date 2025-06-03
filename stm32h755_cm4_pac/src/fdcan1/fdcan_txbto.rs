#[doc = "Register `FDCAN_TXBTO` reader"]
pub type R = crate::R<FdcanTxbtoSpec>;
#[doc = "Register `FDCAN_TXBTO` writer"]
pub type W = crate::W<FdcanTxbtoSpec>;
#[doc = "Field `TO` reader - Transmission Occurred."]
pub type ToR = crate::FieldReader<u32>;
#[doc = "Field `TO` writer - Transmission Occurred."]
pub type ToW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmission Occurred."]
    #[inline(always)]
    pub fn to(&self) -> ToR {
        ToR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmission Occurred."]
    #[inline(always)]
    pub fn to(&mut self) -> ToW<FdcanTxbtoSpec> {
        ToW::new(self, 0)
    }
}
#[doc = "FDCAN Tx Buffer Transmission Occurred Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbto::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbto::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTxbtoSpec;
impl crate::RegisterSpec for FdcanTxbtoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbto::R`](R) reader structure"]
impl crate::Readable for FdcanTxbtoSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txbto::W`](W) writer structure"]
impl crate::Writable for FdcanTxbtoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TXBTO to value 0"]
impl crate::Resettable for FdcanTxbtoSpec {}
