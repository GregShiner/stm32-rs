#[doc = "Register `TIM17_TISEL` reader"]
pub type R = crate::R<Tim17TiselSpec>;
#[doc = "Register `TIM17_TISEL` writer"]
pub type W = crate::W<Tim17TiselSpec>;
#[doc = "Field `TI1SEL` reader - selects TI1\\[0\\] to TI1\\[15\\] input"]
pub type Ti1selR = crate::FieldReader;
#[doc = "Field `TI1SEL` writer - selects TI1\\[0\\] to TI1\\[15\\] input"]
pub type Ti1selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - selects TI1\\[0\\] to TI1\\[15\\] input"]
    #[inline(always)]
    pub fn ti1sel(&self) -> Ti1selR {
        Ti1selR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - selects TI1\\[0\\] to TI1\\[15\\] input"]
    #[inline(always)]
    pub fn ti1sel(&mut self) -> Ti1selW<Tim17TiselSpec> {
        Ti1selW::new(self, 0)
    }
}
#[doc = "TIM17 input selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim17_tisel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_tisel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim17TiselSpec;
impl crate::RegisterSpec for Tim17TiselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim17_tisel::R`](R) reader structure"]
impl crate::Readable for Tim17TiselSpec {}
#[doc = "`write(|w| ..)` method takes [`tim17_tisel::W`](W) writer structure"]
impl crate::Writable for Tim17TiselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM17_TISEL to value 0"]
impl crate::Resettable for Tim17TiselSpec {}
