#[doc = "Register `FDCAN_TXBAR` reader"]
pub type R = crate::R<FdcanTxbarSpec>;
#[doc = "Register `FDCAN_TXBAR` writer"]
pub type W = crate::W<FdcanTxbarSpec>;
#[doc = "Field `AR` reader - Add Request"]
pub type ArR = crate::FieldReader<u32>;
#[doc = "Field `AR` writer - Add Request"]
pub type ArW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Add Request"]
    #[inline(always)]
    pub fn ar(&self) -> ArR {
        ArR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Add Request"]
    #[inline(always)]
    pub fn ar(&mut self) -> ArW<FdcanTxbarSpec> {
        ArW::new(self, 0)
    }
}
#[doc = "FDCAN Tx Buffer Add Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTxbarSpec;
impl crate::RegisterSpec for FdcanTxbarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbar::R`](R) reader structure"]
impl crate::Readable for FdcanTxbarSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txbar::W`](W) writer structure"]
impl crate::Writable for FdcanTxbarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TXBAR to value 0"]
impl crate::Resettable for FdcanTxbarSpec {}
