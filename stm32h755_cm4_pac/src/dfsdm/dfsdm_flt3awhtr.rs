#[doc = "Register `DFSDM_FLT3AWHTR` reader"]
pub type R = crate::R<DfsdmFlt3awhtrSpec>;
#[doc = "Register `DFSDM_FLT3AWHTR` writer"]
pub type W = crate::W<DfsdmFlt3awhtrSpec>;
#[doc = "Field `BKAWH` reader - Break signal assignment to analog watchdog high threshold event"]
pub type BkawhR = crate::FieldReader;
#[doc = "Field `BKAWH` writer - Break signal assignment to analog watchdog high threshold event"]
pub type BkawhW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AWHT` reader - Analog watchdog high threshold"]
pub type AwhtR = crate::FieldReader<u32>;
#[doc = "Field `AWHT` writer - Analog watchdog high threshold"]
pub type AwhtW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog high threshold event"]
    #[inline(always)]
    pub fn bkawh(&self) -> BkawhR {
        BkawhR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - Analog watchdog high threshold"]
    #[inline(always)]
    pub fn awht(&self) -> AwhtR {
        AwhtR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog high threshold event"]
    #[inline(always)]
    pub fn bkawh(&mut self) -> BkawhW<DfsdmFlt3awhtrSpec> {
        BkawhW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Analog watchdog high threshold"]
    #[inline(always)]
    pub fn awht(&mut self) -> AwhtW<DfsdmFlt3awhtrSpec> {
        AwhtW::new(self, 8)
    }
}
#[doc = "analog watchdog high threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3awhtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3awhtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfsdmFlt3awhtrSpec;
impl crate::RegisterSpec for DfsdmFlt3awhtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt3awhtr::R`](R) reader structure"]
impl crate::Readable for DfsdmFlt3awhtrSpec {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_flt3awhtr::W`](W) writer structure"]
impl crate::Writable for DfsdmFlt3awhtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DFSDM_FLT3AWHTR to value 0"]
impl crate::Resettable for DfsdmFlt3awhtrSpec {}
