#[doc = "Register `DFSDM_FLT1FCR` reader"]
pub type R = crate::R<DfsdmFlt1fcrSpec>;
#[doc = "Register `DFSDM_FLT1FCR` writer"]
pub type W = crate::W<DfsdmFlt1fcrSpec>;
#[doc = "Field `IOSR` reader - Integrator oversampling ratio (averaging length)"]
pub type IosrR = crate::FieldReader;
#[doc = "Field `IOSR` writer - Integrator oversampling ratio (averaging length)"]
pub type IosrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FOSR` reader - Sinc filter oversampling ratio (decimation rate)"]
pub type FosrR = crate::FieldReader<u16>;
#[doc = "Field `FOSR` writer - Sinc filter oversampling ratio (decimation rate)"]
pub type FosrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `FORD` reader - Sinc filter order"]
pub type FordR = crate::FieldReader;
#[doc = "Field `FORD` writer - Sinc filter order"]
pub type FordW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - Integrator oversampling ratio (averaging length)"]
    #[inline(always)]
    pub fn iosr(&self) -> IosrR {
        IosrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate)"]
    #[inline(always)]
    pub fn fosr(&self) -> FosrR {
        FosrR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:31 - Sinc filter order"]
    #[inline(always)]
    pub fn ford(&self) -> FordR {
        FordR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Integrator oversampling ratio (averaging length)"]
    #[inline(always)]
    pub fn iosr(&mut self) -> IosrW<DfsdmFlt1fcrSpec> {
        IosrW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate)"]
    #[inline(always)]
    pub fn fosr(&mut self) -> FosrW<DfsdmFlt1fcrSpec> {
        FosrW::new(self, 16)
    }
    #[doc = "Bits 29:31 - Sinc filter order"]
    #[inline(always)]
    pub fn ford(&mut self) -> FordW<DfsdmFlt1fcrSpec> {
        FordW::new(self, 29)
    }
}
#[doc = "filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfsdmFlt1fcrSpec;
impl crate::RegisterSpec for DfsdmFlt1fcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt1fcr::R`](R) reader structure"]
impl crate::Readable for DfsdmFlt1fcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_flt1fcr::W`](W) writer structure"]
impl crate::Writable for DfsdmFlt1fcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DFSDM_FLT1FCR to value 0"]
impl crate::Resettable for DfsdmFlt1fcrSpec {}
