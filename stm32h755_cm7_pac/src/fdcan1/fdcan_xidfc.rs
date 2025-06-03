#[doc = "Register `FDCAN_XIDFC` reader"]
pub type R = crate::R<FdcanXidfcSpec>;
#[doc = "Register `FDCAN_XIDFC` writer"]
pub type W = crate::W<FdcanXidfcSpec>;
#[doc = "Field `FLESA` reader - Filter List Standard Start Address"]
pub type FlesaR = crate::FieldReader<u16>;
#[doc = "Field `FLESA` writer - Filter List Standard Start Address"]
pub type FlesaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `LSE` reader - List Size Extended"]
pub type LseR = crate::FieldReader;
#[doc = "Field `LSE` writer - List Size Extended"]
pub type LseW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 2:15 - Filter List Standard Start Address"]
    #[inline(always)]
    pub fn flesa(&self) -> FlesaR {
        FlesaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:23 - List Size Extended"]
    #[inline(always)]
    pub fn lse(&self) -> LseR {
        LseR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Filter List Standard Start Address"]
    #[inline(always)]
    pub fn flesa(&mut self) -> FlesaW<FdcanXidfcSpec> {
        FlesaW::new(self, 2)
    }
    #[doc = "Bits 16:23 - List Size Extended"]
    #[inline(always)]
    pub fn lse(&mut self) -> LseW<FdcanXidfcSpec> {
        LseW::new(self, 16)
    }
}
#[doc = "FDCAN Extended ID Filter Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_xidfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_xidfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanXidfcSpec;
impl crate::RegisterSpec for FdcanXidfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_xidfc::R`](R) reader structure"]
impl crate::Readable for FdcanXidfcSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_xidfc::W`](W) writer structure"]
impl crate::Writable for FdcanXidfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_XIDFC to value 0"]
impl crate::Resettable for FdcanXidfcSpec {}
