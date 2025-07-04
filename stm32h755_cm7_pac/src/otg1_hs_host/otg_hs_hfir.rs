#[doc = "Register `OTG_HS_HFIR` reader"]
pub type R = crate::R<OtgHsHfirSpec>;
#[doc = "Register `OTG_HS_HFIR` writer"]
pub type W = crate::W<OtgHsHfirSpec>;
#[doc = "Field `FRIVL` reader - Frame interval"]
pub type FrivlR = crate::FieldReader<u16>;
#[doc = "Field `FRIVL` writer - Frame interval"]
pub type FrivlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Frame interval"]
    #[inline(always)]
    pub fn frivl(&self) -> FrivlR {
        FrivlR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame interval"]
    #[inline(always)]
    pub fn frivl(&mut self) -> FrivlW<OtgHsHfirSpec> {
        FrivlW::new(self, 0)
    }
}
#[doc = "OTG_HS Host frame interval register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_hfir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hfir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsHfirSpec;
impl crate::RegisterSpec for OtgHsHfirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_hfir::R`](R) reader structure"]
impl crate::Readable for OtgHsHfirSpec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_hfir::W`](W) writer structure"]
impl crate::Writable for OtgHsHfirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_HFIR to value 0xea60"]
impl crate::Resettable for OtgHsHfirSpec {
    const RESET_VALUE: u32 = 0xea60;
}
