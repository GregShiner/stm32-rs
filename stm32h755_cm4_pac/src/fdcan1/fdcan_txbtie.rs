#[doc = "Register `FDCAN_TXBTIE` reader"]
pub type R = crate::R<FdcanTxbtieSpec>;
#[doc = "Register `FDCAN_TXBTIE` writer"]
pub type W = crate::W<FdcanTxbtieSpec>;
#[doc = "Field `TIE` reader - Transmission Interrupt Enable"]
pub type TieR = crate::FieldReader<u32>;
#[doc = "Field `TIE` writer - Transmission Interrupt Enable"]
pub type TieW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmission Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmission Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<FdcanTxbtieSpec> {
        TieW::new(self, 0)
    }
}
#[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbtie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbtie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTxbtieSpec;
impl crate::RegisterSpec for FdcanTxbtieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbtie::R`](R) reader structure"]
impl crate::Readable for FdcanTxbtieSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txbtie::W`](W) writer structure"]
impl crate::Writable for FdcanTxbtieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TXBTIE to value 0"]
impl crate::Resettable for FdcanTxbtieSpec {}
