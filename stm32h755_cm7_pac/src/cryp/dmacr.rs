#[doc = "Register `DMACR` reader"]
pub type R = crate::R<DmacrSpec>;
#[doc = "Register `DMACR` writer"]
pub type W = crate::W<DmacrSpec>;
#[doc = "Field `DIEN` reader - DMA input enable"]
pub type DienR = crate::BitReader;
#[doc = "Field `DIEN` writer - DMA input enable"]
pub type DienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOEN` reader - DMA output enable"]
pub type DoenR = crate::BitReader;
#[doc = "Field `DOEN` writer - DMA output enable"]
pub type DoenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA input enable"]
    #[inline(always)]
    pub fn dien(&self) -> DienR {
        DienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA output enable"]
    #[inline(always)]
    pub fn doen(&self) -> DoenR {
        DoenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA input enable"]
    #[inline(always)]
    pub fn dien(&mut self) -> DienW<DmacrSpec> {
        DienW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA output enable"]
    #[inline(always)]
    pub fn doen(&mut self) -> DoenW<DmacrSpec> {
        DoenW::new(self, 1)
    }
}
#[doc = "DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacrSpec;
impl crate::RegisterSpec for DmacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacr::R`](R) reader structure"]
impl crate::Readable for DmacrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacr::W`](W) writer structure"]
impl crate::Writable for DmacrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACR to value 0"]
impl crate::Resettable for DmacrSpec {}
