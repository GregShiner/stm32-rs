#[doc = "Register `FDCAN_TURCF` reader"]
pub type R = crate::R<FdcanTurcfSpec>;
#[doc = "Register `FDCAN_TURCF` writer"]
pub type W = crate::W<FdcanTurcfSpec>;
#[doc = "Field `NCL` reader - Numerator Configuration Low."]
pub type NclR = crate::FieldReader<u16>;
#[doc = "Field `NCL` writer - Numerator Configuration Low."]
pub type NclW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DC` reader - Denominator Configuration."]
pub type DcR = crate::FieldReader<u16>;
#[doc = "Field `DC` writer - Denominator Configuration."]
pub type DcW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `ELT` reader - Enable Local Time"]
pub type EltR = crate::BitReader;
#[doc = "Field `ELT` writer - Enable Local Time"]
pub type EltW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Numerator Configuration Low."]
    #[inline(always)]
    pub fn ncl(&self) -> NclR {
        NclR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:29 - Denominator Configuration."]
    #[inline(always)]
    pub fn dc(&self) -> DcR {
        DcR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Enable Local Time"]
    #[inline(always)]
    pub fn elt(&self) -> EltR {
        EltR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Numerator Configuration Low."]
    #[inline(always)]
    pub fn ncl(&mut self) -> NclW<FdcanTurcfSpec> {
        NclW::new(self, 0)
    }
    #[doc = "Bits 16:29 - Denominator Configuration."]
    #[inline(always)]
    pub fn dc(&mut self) -> DcW<FdcanTurcfSpec> {
        DcW::new(self, 16)
    }
    #[doc = "Bit 31 - Enable Local Time"]
    #[inline(always)]
    pub fn elt(&mut self) -> EltW<FdcanTurcfSpec> {
        EltW::new(self, 31)
    }
}
#[doc = "FDCAN TUR Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_turcf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_turcf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTurcfSpec;
impl crate::RegisterSpec for FdcanTurcfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_turcf::R`](R) reader structure"]
impl crate::Readable for FdcanTurcfSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_turcf::W`](W) writer structure"]
impl crate::Writable for FdcanTurcfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TURCF to value 0"]
impl crate::Resettable for FdcanTurcfSpec {}
