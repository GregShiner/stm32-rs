#[doc = "Register `OPAMP1_OTR` reader"]
pub type R = crate::R<Opamp1OtrSpec>;
#[doc = "Register `OPAMP1_OTR` writer"]
pub type W = crate::W<Opamp1OtrSpec>;
#[doc = "Field `TRIMOFFSETN` reader - Trim for NMOS differential pairs"]
pub type TrimoffsetnR = crate::FieldReader;
#[doc = "Field `TRIMOFFSETN` writer - Trim for NMOS differential pairs"]
pub type TrimoffsetnW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIMOFFSETP` reader - Trim for PMOS differential pairs"]
pub type TrimoffsetpR = crate::FieldReader;
#[doc = "Field `TRIMOFFSETP` writer - Trim for PMOS differential pairs"]
pub type TrimoffsetpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TrimoffsetnR {
        TrimoffsetnR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TrimoffsetpR {
        TrimoffsetpR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn trimoffsetn(&mut self) -> TrimoffsetnW<Opamp1OtrSpec> {
        TrimoffsetnW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn trimoffsetp(&mut self) -> TrimoffsetpW<Opamp1OtrSpec> {
        TrimoffsetpW::new(self, 8)
    }
}
#[doc = "OPAMP1 offset trimming register in normal mode\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp1_otr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_otr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Opamp1OtrSpec;
impl crate::RegisterSpec for Opamp1OtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp1_otr::R`](R) reader structure"]
impl crate::Readable for Opamp1OtrSpec {}
#[doc = "`write(|w| ..)` method takes [`opamp1_otr::W`](W) writer structure"]
impl crate::Writable for Opamp1OtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPAMP1_OTR to value 0"]
impl crate::Resettable for Opamp1OtrSpec {}
