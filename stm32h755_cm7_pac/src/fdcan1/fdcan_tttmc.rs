#[doc = "Register `FDCAN_TTTMC` reader"]
pub type R = crate::R<FdcanTttmcSpec>;
#[doc = "Register `FDCAN_TTTMC` writer"]
pub type W = crate::W<FdcanTttmcSpec>;
#[doc = "Field `TMSA` reader - Trigger Memory Start Address"]
pub type TmsaR = crate::FieldReader<u16>;
#[doc = "Field `TMSA` writer - Trigger Memory Start Address"]
pub type TmsaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `TME` reader - Trigger Memory Elements"]
pub type TmeR = crate::FieldReader;
#[doc = "Field `TME` writer - Trigger Memory Elements"]
pub type TmeW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 2:15 - Trigger Memory Start Address"]
    #[inline(always)]
    pub fn tmsa(&self) -> TmsaR {
        TmsaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - Trigger Memory Elements"]
    #[inline(always)]
    pub fn tme(&self) -> TmeR {
        TmeR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Trigger Memory Start Address"]
    #[inline(always)]
    pub fn tmsa(&mut self) -> TmsaW<FdcanTttmcSpec> {
        TmsaW::new(self, 2)
    }
    #[doc = "Bits 16:22 - Trigger Memory Elements"]
    #[inline(always)]
    pub fn tme(&mut self) -> TmeW<FdcanTttmcSpec> {
        TmeW::new(self, 16)
    }
}
#[doc = "FDCAN TT Trigger Memory Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_tttmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tttmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTttmcSpec;
impl crate::RegisterSpec for FdcanTttmcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_tttmc::R`](R) reader structure"]
impl crate::Readable for FdcanTttmcSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_tttmc::W`](W) writer structure"]
impl crate::Writable for FdcanTttmcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TTTMC to value 0"]
impl crate::Resettable for FdcanTttmcSpec {}
