#[doc = "Register `BPCR` reader"]
pub type R = crate::R<BpcrSpec>;
#[doc = "Register `BPCR` writer"]
pub type W = crate::W<BpcrSpec>;
#[doc = "Field `AVBP` reader - Accumulated Vertical back porch (in units of horizontal scan line)"]
pub type AvbpR = crate::FieldReader<u16>;
#[doc = "Field `AVBP` writer - Accumulated Vertical back porch (in units of horizontal scan line)"]
pub type AvbpW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `AHBP` reader - Accumulated Horizontal back porch (in units of pixel clock period)"]
pub type AhbpR = crate::FieldReader<u16>;
#[doc = "Field `AHBP` writer - Accumulated Horizontal back porch (in units of pixel clock period)"]
pub type AhbpW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:10 - Accumulated Vertical back porch (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn avbp(&self) -> AvbpR {
        AvbpR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:27 - Accumulated Horizontal back porch (in units of pixel clock period)"]
    #[inline(always)]
    pub fn ahbp(&self) -> AhbpR {
        AhbpR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Accumulated Vertical back porch (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn avbp(&mut self) -> AvbpW<BpcrSpec> {
        AvbpW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Accumulated Horizontal back porch (in units of pixel clock period)"]
    #[inline(always)]
    pub fn ahbp(&mut self) -> AhbpW<BpcrSpec> {
        AhbpW::new(self, 16)
    }
}
#[doc = "Back Porch Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BpcrSpec;
impl crate::RegisterSpec for BpcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bpcr::R`](R) reader structure"]
impl crate::Readable for BpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`bpcr::W`](W) writer structure"]
impl crate::Writable for BpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BPCR to value 0"]
impl crate::Resettable for BpcrSpec {}
