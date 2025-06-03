#[doc = "Register `CCCR` reader"]
pub type R = crate::R<CccrSpec>;
#[doc = "Register `CCCR` writer"]
pub type W = crate::W<CccrSpec>;
#[doc = "Field `NCC` reader - NMOS compensation code"]
pub type NccR = crate::FieldReader;
#[doc = "Field `NCC` writer - NMOS compensation code"]
pub type NccW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PCC` reader - PMOS compensation code"]
pub type PccR = crate::FieldReader;
#[doc = "Field `PCC` writer - PMOS compensation code"]
pub type PccW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - NMOS compensation code"]
    #[inline(always)]
    pub fn ncc(&self) -> NccR {
        NccR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PMOS compensation code"]
    #[inline(always)]
    pub fn pcc(&self) -> PccR {
        PccR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - NMOS compensation code"]
    #[inline(always)]
    pub fn ncc(&mut self) -> NccW<CccrSpec> {
        NccW::new(self, 0)
    }
    #[doc = "Bits 4:7 - PMOS compensation code"]
    #[inline(always)]
    pub fn pcc(&mut self) -> PccW<CccrSpec> {
        PccW::new(self, 4)
    }
}
#[doc = "SYSCFG compensation cell code register\n\nYou can [`read`](crate::Reg::read) this register and get [`cccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CccrSpec;
impl crate::RegisterSpec for CccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cccr::R`](R) reader structure"]
impl crate::Readable for CccrSpec {}
#[doc = "`write(|w| ..)` method takes [`cccr::W`](W) writer structure"]
impl crate::Writable for CccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCCR to value 0"]
impl crate::Resettable for CccrSpec {}
