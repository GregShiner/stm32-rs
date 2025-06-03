#[doc = "Register `DMASBMR` reader"]
pub type R = crate::R<DmasbmrSpec>;
#[doc = "Register `DMASBMR` writer"]
pub type W = crate::W<DmasbmrSpec>;
#[doc = "Field `FB` reader - Fixed Burst Length"]
pub type FbR = crate::BitReader;
#[doc = "Field `FB` writer - Fixed Burst Length"]
pub type FbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAL` reader - Address-Aligned Beats"]
pub type AalR = crate::BitReader;
#[doc = "Field `AAL` writer - Address-Aligned Beats"]
pub type AalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB` reader - Mixed Burst"]
pub type MbR = crate::BitReader;
#[doc = "Field `RB` reader - Rebuild INCRx Burst"]
pub type RbR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Fixed Burst Length"]
    #[inline(always)]
    pub fn fb(&self) -> FbR {
        FbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 12 - Address-Aligned Beats"]
    #[inline(always)]
    pub fn aal(&self) -> AalR {
        AalR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Mixed Burst"]
    #[inline(always)]
    pub fn mb(&self) -> MbR {
        MbR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rebuild INCRx Burst"]
    #[inline(always)]
    pub fn rb(&self) -> RbR {
        RbR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fixed Burst Length"]
    #[inline(always)]
    pub fn fb(&mut self) -> FbW<DmasbmrSpec> {
        FbW::new(self, 0)
    }
    #[doc = "Bit 12 - Address-Aligned Beats"]
    #[inline(always)]
    pub fn aal(&mut self) -> AalW<DmasbmrSpec> {
        AalW::new(self, 12)
    }
}
#[doc = "System bus mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasbmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasbmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmasbmrSpec;
impl crate::RegisterSpec for DmasbmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasbmr::R`](R) reader structure"]
impl crate::Readable for DmasbmrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmasbmr::W`](W) writer structure"]
impl crate::Writable for DmasbmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMASBMR to value 0x0101_0000"]
impl crate::Resettable for DmasbmrSpec {
    const RESET_VALUE: u32 = 0x0101_0000;
}
