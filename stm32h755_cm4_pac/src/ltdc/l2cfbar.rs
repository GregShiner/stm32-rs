#[doc = "Register `L2CFBAR` reader"]
pub type R = crate::R<L2cfbarSpec>;
#[doc = "Register `L2CFBAR` writer"]
pub type W = crate::W<L2cfbarSpec>;
#[doc = "Field `CFBADD` reader - Color Frame Buffer Start Address"]
pub type CfbaddR = crate::FieldReader<u32>;
#[doc = "Field `CFBADD` writer - Color Frame Buffer Start Address"]
pub type CfbaddW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Color Frame Buffer Start Address"]
    #[inline(always)]
    pub fn cfbadd(&self) -> CfbaddR {
        CfbaddR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Color Frame Buffer Start Address"]
    #[inline(always)]
    pub fn cfbadd(&mut self) -> CfbaddW<L2cfbarSpec> {
        CfbaddW::new(self, 0)
    }
}
#[doc = "Layerx Color Frame Buffer Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2cfbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cfbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2cfbarSpec;
impl crate::RegisterSpec for L2cfbarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2cfbar::R`](R) reader structure"]
impl crate::Readable for L2cfbarSpec {}
#[doc = "`write(|w| ..)` method takes [`l2cfbar::W`](W) writer structure"]
impl crate::Writable for L2cfbarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2CFBAR to value 0"]
impl crate::Resettable for L2cfbarSpec {}
