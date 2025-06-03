#[doc = "Register `L1CLUTWR` writer"]
pub type W = crate::W<L1clutwrSpec>;
#[doc = "Field `BLUE` writer - Blue value"]
pub type BlueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GREEN` writer - Green value"]
pub type GreenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED` writer - Red value"]
pub type RedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLUTADD` writer - CLUT Address"]
pub type ClutaddW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Blue value"]
    #[inline(always)]
    pub fn blue(&mut self) -> BlueW<L1clutwrSpec> {
        BlueW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Green value"]
    #[inline(always)]
    pub fn green(&mut self) -> GreenW<L1clutwrSpec> {
        GreenW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Red value"]
    #[inline(always)]
    pub fn red(&mut self) -> RedW<L1clutwrSpec> {
        RedW::new(self, 16)
    }
    #[doc = "Bits 24:31 - CLUT Address"]
    #[inline(always)]
    pub fn clutadd(&mut self) -> ClutaddW<L1clutwrSpec> {
        ClutaddW::new(self, 24)
    }
}
#[doc = "Layerx CLUT Write Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1clutwr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1clutwrSpec;
impl crate::RegisterSpec for L1clutwrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`l1clutwr::W`](W) writer structure"]
impl crate::Writable for L1clutwrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1CLUTWR to value 0"]
impl crate::Resettable for L1clutwrSpec {}
