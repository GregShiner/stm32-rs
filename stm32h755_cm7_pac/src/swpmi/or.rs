#[doc = "Register `OR` reader"]
pub type R = crate::R<OrSpec>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<OrSpec>;
#[doc = "Field `SWP_TBYP` reader - SWP transceiver bypass"]
pub type SwpTbypR = crate::BitReader;
#[doc = "Field `SWP_TBYP` writer - SWP transceiver bypass"]
pub type SwpTbypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWP_CLASS` reader - SWP class selection"]
pub type SwpClassR = crate::BitReader;
#[doc = "Field `SWP_CLASS` writer - SWP class selection"]
pub type SwpClassW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SWP transceiver bypass"]
    #[inline(always)]
    pub fn swp_tbyp(&self) -> SwpTbypR {
        SwpTbypR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SWP class selection"]
    #[inline(always)]
    pub fn swp_class(&self) -> SwpClassR {
        SwpClassR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SWP transceiver bypass"]
    #[inline(always)]
    pub fn swp_tbyp(&mut self) -> SwpTbypW<OrSpec> {
        SwpTbypW::new(self, 0)
    }
    #[doc = "Bit 1 - SWP class selection"]
    #[inline(always)]
    pub fn swp_class(&mut self) -> SwpClassW<OrSpec> {
        SwpClassW::new(self, 1)
    }
}
#[doc = "SWPMI Option register\n\nYou can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OrSpec;
impl crate::RegisterSpec for OrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for OrSpec {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for OrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OrSpec {}
