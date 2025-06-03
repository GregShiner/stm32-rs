#[doc = "Register `TISEL` reader"]
pub type R = crate::R<TiselSpec>;
#[doc = "Register `TISEL` writer"]
pub type W = crate::W<TiselSpec>;
#[doc = "Field `TI1SEL` reader - selects TI1\\[0\\] to TI1\\[15\\] input"]
pub type Ti1selR = crate::FieldReader;
#[doc = "Field `TI1SEL` writer - selects TI1\\[0\\] to TI1\\[15\\] input"]
pub type Ti1selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TI2SEL` reader - selects TI2\\[0\\] to TI2\\[15\\] input"]
pub type Ti2selR = crate::FieldReader;
#[doc = "Field `TI2SEL` writer - selects TI2\\[0\\] to TI2\\[15\\] input"]
pub type Ti2selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - selects TI1\\[0\\] to TI1\\[15\\] input"]
    #[inline(always)]
    pub fn ti1sel(&self) -> Ti1selR {
        Ti1selR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - selects TI2\\[0\\] to TI2\\[15\\] input"]
    #[inline(always)]
    pub fn ti2sel(&self) -> Ti2selR {
        Ti2selR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - selects TI1\\[0\\] to TI1\\[15\\] input"]
    #[inline(always)]
    pub fn ti1sel(&mut self) -> Ti1selW<TiselSpec> {
        Ti1selW::new(self, 0)
    }
    #[doc = "Bits 8:11 - selects TI2\\[0\\] to TI2\\[15\\] input"]
    #[inline(always)]
    pub fn ti2sel(&mut self) -> Ti2selW<TiselSpec> {
        Ti2selW::new(self, 8)
    }
}
#[doc = "TIM15 input selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`tisel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TiselSpec;
impl crate::RegisterSpec for TiselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tisel::R`](R) reader structure"]
impl crate::Readable for TiselSpec {}
#[doc = "`write(|w| ..)` method takes [`tisel::W`](W) writer structure"]
impl crate::Writable for TiselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TISEL to value 0"]
impl crate::Resettable for TiselSpec {}
