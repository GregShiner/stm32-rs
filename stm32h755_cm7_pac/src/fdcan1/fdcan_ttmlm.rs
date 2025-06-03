#[doc = "Register `FDCAN_TTMLM` reader"]
pub type R = crate::R<FdcanTtmlmSpec>;
#[doc = "Register `FDCAN_TTMLM` writer"]
pub type W = crate::W<FdcanTtmlmSpec>;
#[doc = "Field `CCM` reader - Cycle Count Max"]
pub type CcmR = crate::FieldReader;
#[doc = "Field `CCM` writer - Cycle Count Max"]
pub type CcmW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CSS` reader - Cycle Start Synchronization"]
pub type CssR = crate::FieldReader;
#[doc = "Field `CSS` writer - Cycle Start Synchronization"]
pub type CssW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXEW` reader - Tx Enable Window"]
pub type TxewR = crate::FieldReader;
#[doc = "Field `TXEW` writer - Tx Enable Window"]
pub type TxewW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ENTT` reader - Expected Number of Tx Triggers"]
pub type EnttR = crate::FieldReader<u16>;
#[doc = "Field `ENTT` writer - Expected Number of Tx Triggers"]
pub type EnttW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:5 - Cycle Count Max"]
    #[inline(always)]
    pub fn ccm(&self) -> CcmR {
        CcmR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Cycle Start Synchronization"]
    #[inline(always)]
    pub fn css(&self) -> CssR {
        CssR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Tx Enable Window"]
    #[inline(always)]
    pub fn txew(&self) -> TxewR {
        TxewR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - Expected Number of Tx Triggers"]
    #[inline(always)]
    pub fn entt(&self) -> EnttR {
        EnttR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - Cycle Count Max"]
    #[inline(always)]
    pub fn ccm(&mut self) -> CcmW<FdcanTtmlmSpec> {
        CcmW::new(self, 0)
    }
    #[doc = "Bits 6:7 - Cycle Start Synchronization"]
    #[inline(always)]
    pub fn css(&mut self) -> CssW<FdcanTtmlmSpec> {
        CssW::new(self, 6)
    }
    #[doc = "Bits 8:11 - Tx Enable Window"]
    #[inline(always)]
    pub fn txew(&mut self) -> TxewW<FdcanTtmlmSpec> {
        TxewW::new(self, 8)
    }
    #[doc = "Bits 16:27 - Expected Number of Tx Triggers"]
    #[inline(always)]
    pub fn entt(&mut self) -> EnttW<FdcanTtmlmSpec> {
        EnttW::new(self, 16)
    }
}
#[doc = "FDCAN TT Matrix Limits Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttmlm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttmlm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTtmlmSpec;
impl crate::RegisterSpec for FdcanTtmlmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttmlm::R`](R) reader structure"]
impl crate::Readable for FdcanTtmlmSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttmlm::W`](W) writer structure"]
impl crate::Writable for FdcanTtmlmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TTMLM to value 0"]
impl crate::Resettable for FdcanTtmlmSpec {}
