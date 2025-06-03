#[doc = "Register `L1CFBAR` reader"]
pub type R = crate::R<L1cfbarSpec>;
#[doc = "Register `L1CFBAR` writer"]
pub type W = crate::W<L1cfbarSpec>;
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
    pub fn cfbadd(&mut self) -> CfbaddW<L1cfbarSpec> {
        CfbaddW::new(self, 0)
    }
}
#[doc = "Layerx Color Frame Buffer Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1cfbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cfbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1cfbarSpec;
impl crate::RegisterSpec for L1cfbarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1cfbar::R`](R) reader structure"]
impl crate::Readable for L1cfbarSpec {}
#[doc = "`write(|w| ..)` method takes [`l1cfbar::W`](W) writer structure"]
impl crate::Writable for L1cfbarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1CFBAR to value 0"]
impl crate::Resettable for L1cfbarSpec {}
