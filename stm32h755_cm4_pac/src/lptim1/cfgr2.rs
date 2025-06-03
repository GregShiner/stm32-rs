#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<Cfgr2Spec>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<Cfgr2Spec>;
#[doc = "Field `IN1SEL` reader - LPTIM Input 1 selection"]
pub type In1selR = crate::FieldReader;
#[doc = "Field `IN1SEL` writer - LPTIM Input 1 selection"]
pub type In1selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IN2SEL` reader - LPTIM Input 2 selection"]
pub type In2selR = crate::FieldReader;
#[doc = "Field `IN2SEL` writer - LPTIM Input 2 selection"]
pub type In2selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - LPTIM Input 1 selection"]
    #[inline(always)]
    pub fn in1sel(&self) -> In1selR {
        In1selR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - LPTIM Input 2 selection"]
    #[inline(always)]
    pub fn in2sel(&self) -> In2selR {
        In2selR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - LPTIM Input 1 selection"]
    #[inline(always)]
    pub fn in1sel(&mut self) -> In1selW<Cfgr2Spec> {
        In1selW::new(self, 0)
    }
    #[doc = "Bits 4:5 - LPTIM Input 2 selection"]
    #[inline(always)]
    pub fn in2sel(&mut self) -> In2selW<Cfgr2Spec> {
        In2selW::new(self, 4)
    }
}
#[doc = "LPTIM configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr2Spec;
impl crate::RegisterSpec for Cfgr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for Cfgr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for Cfgr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for Cfgr2Spec {}
