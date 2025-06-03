#[doc = "Register `OTG_HS_DIEPEMPMSK` reader"]
pub type R = crate::R<OtgHsDiepempmskSpec>;
#[doc = "Register `OTG_HS_DIEPEMPMSK` writer"]
pub type W = crate::W<OtgHsDiepempmskSpec>;
#[doc = "Field `INEPTXFEM` reader - IN EP Tx FIFO empty interrupt mask bits"]
pub type IneptxfemR = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFEM` writer - IN EP Tx FIFO empty interrupt mask bits"]
pub type IneptxfemW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN EP Tx FIFO empty interrupt mask bits"]
    #[inline(always)]
    pub fn ineptxfem(&self) -> IneptxfemR {
        IneptxfemR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP Tx FIFO empty interrupt mask bits"]
    #[inline(always)]
    pub fn ineptxfem(&mut self) -> IneptxfemW<OtgHsDiepempmskSpec> {
        IneptxfemW::new(self, 0)
    }
}
#[doc = "OTG_HS device IN endpoint FIFO empty interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_diepempmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepempmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsDiepempmskSpec;
impl crate::RegisterSpec for OtgHsDiepempmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_diepempmsk::R`](R) reader structure"]
impl crate::Readable for OtgHsDiepempmskSpec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_diepempmsk::W`](W) writer structure"]
impl crate::Writable for OtgHsDiepempmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_DIEPEMPMSK to value 0"]
impl crate::Resettable for OtgHsDiepempmskSpec {}
