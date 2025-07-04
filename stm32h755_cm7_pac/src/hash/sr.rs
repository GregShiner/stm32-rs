#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `DINIS` reader - Data input interrupt status"]
pub type DinisR = crate::BitReader;
#[doc = "Field `DINIS` writer - Data input interrupt status"]
pub type DinisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIS` reader - Digest calculation completion interrupt status"]
pub type DcisR = crate::BitReader;
#[doc = "Field `DCIS` writer - Digest calculation completion interrupt status"]
pub type DcisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAS` reader - DMA Status"]
pub type DmasR = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy bit"]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data input interrupt status"]
    #[inline(always)]
    pub fn dinis(&self) -> DinisR {
        DinisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt status"]
    #[inline(always)]
    pub fn dcis(&self) -> DcisR {
        DcisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Status"]
    #[inline(always)]
    pub fn dmas(&self) -> DmasR {
        DmasR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Busy bit"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data input interrupt status"]
    #[inline(always)]
    pub fn dinis(&mut self) -> DinisW<SrSpec> {
        DinisW::new(self, 0)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt status"]
    #[inline(always)]
    pub fn dcis(&mut self) -> DcisW<SrSpec> {
        DcisW::new(self, 1)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0x01"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x01;
}
