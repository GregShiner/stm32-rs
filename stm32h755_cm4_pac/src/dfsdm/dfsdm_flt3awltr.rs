#[doc = "Register `DFSDM_FLT3AWLTR` reader"]
pub type R = crate::R<DfsdmFlt3awltrSpec>;
#[doc = "Register `DFSDM_FLT3AWLTR` writer"]
pub type W = crate::W<DfsdmFlt3awltrSpec>;
#[doc = "Field `BKAWL` reader - Break signal assignment to analog watchdog low threshold event"]
pub type BkawlR = crate::FieldReader;
#[doc = "Field `BKAWL` writer - Break signal assignment to analog watchdog low threshold event"]
pub type BkawlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AWLT` reader - Analog watchdog low threshold"]
pub type AwltR = crate::FieldReader<u32>;
#[doc = "Field `AWLT` writer - Analog watchdog low threshold"]
pub type AwltW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event"]
    #[inline(always)]
    pub fn bkawl(&self) -> BkawlR {
        BkawlR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - Analog watchdog low threshold"]
    #[inline(always)]
    pub fn awlt(&self) -> AwltR {
        AwltR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event"]
    #[inline(always)]
    pub fn bkawl(&mut self) -> BkawlW<DfsdmFlt3awltrSpec> {
        BkawlW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Analog watchdog low threshold"]
    #[inline(always)]
    pub fn awlt(&mut self) -> AwltW<DfsdmFlt3awltrSpec> {
        AwltW::new(self, 8)
    }
}
#[doc = "analog watchdog low threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3awltr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3awltr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfsdmFlt3awltrSpec;
impl crate::RegisterSpec for DfsdmFlt3awltrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt3awltr::R`](R) reader structure"]
impl crate::Readable for DfsdmFlt3awltrSpec {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_flt3awltr::W`](W) writer structure"]
impl crate::Writable for DfsdmFlt3awltrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DFSDM_FLT3AWLTR to value 0"]
impl crate::Resettable for DfsdmFlt3awltrSpec {}
