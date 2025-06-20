#[doc = "Register `OTG_HS_DAINTMSK` reader"]
pub type R = crate::R<OtgHsDaintmskSpec>;
#[doc = "Register `OTG_HS_DAINTMSK` writer"]
pub type W = crate::W<OtgHsDaintmskSpec>;
#[doc = "Field `IEPM` reader - IN EP interrupt mask bits"]
pub type IepmR = crate::FieldReader<u16>;
#[doc = "Field `IEPM` writer - IN EP interrupt mask bits"]
pub type IepmW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OEPM` reader - OUT EP interrupt mask bits"]
pub type OepmR = crate::FieldReader<u16>;
#[doc = "Field `OEPM` writer - OUT EP interrupt mask bits"]
pub type OepmW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN EP interrupt mask bits"]
    #[inline(always)]
    pub fn iepm(&self) -> IepmR {
        IepmR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT EP interrupt mask bits"]
    #[inline(always)]
    pub fn oepm(&self) -> OepmR {
        OepmR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP interrupt mask bits"]
    #[inline(always)]
    pub fn iepm(&mut self) -> IepmW<OtgHsDaintmskSpec> {
        IepmW::new(self, 0)
    }
    #[doc = "Bits 16:31 - OUT EP interrupt mask bits"]
    #[inline(always)]
    pub fn oepm(&mut self) -> OepmW<OtgHsDaintmskSpec> {
        OepmW::new(self, 16)
    }
}
#[doc = "OTG_HS all endpoints interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_daintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_daintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsDaintmskSpec;
impl crate::RegisterSpec for OtgHsDaintmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_daintmsk::R`](R) reader structure"]
impl crate::Readable for OtgHsDaintmskSpec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_daintmsk::W`](W) writer structure"]
impl crate::Writable for OtgHsDaintmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_DAINTMSK to value 0"]
impl crate::Resettable for OtgHsDaintmskSpec {}
