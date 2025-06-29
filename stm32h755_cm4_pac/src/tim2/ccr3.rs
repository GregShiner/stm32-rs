#[doc = "Register `CCR3` reader"]
pub type R = crate::R<Ccr3Spec>;
#[doc = "Register `CCR3` writer"]
pub type W = crate::W<Ccr3Spec>;
#[doc = "Field `CCR3_L` reader - Low Capture/Compare value"]
pub type Ccr3LR = crate::FieldReader<u16>;
#[doc = "Field `CCR3_L` writer - Low Capture/Compare value"]
pub type Ccr3LW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CCR3_H` reader - High Capture/Compare value"]
pub type Ccr3HR = crate::FieldReader<u16>;
#[doc = "Field `CCR3_H` writer - High Capture/Compare value"]
pub type Ccr3HW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low Capture/Compare value"]
    #[inline(always)]
    pub fn ccr3_l(&self) -> Ccr3LR {
        Ccr3LR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Capture/Compare value"]
    #[inline(always)]
    pub fn ccr3_h(&self) -> Ccr3HR {
        Ccr3HR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Capture/Compare value"]
    #[inline(always)]
    pub fn ccr3_l(&mut self) -> Ccr3LW<Ccr3Spec> {
        Ccr3LW::new(self, 0)
    }
    #[doc = "Bits 16:31 - High Capture/Compare value"]
    #[inline(always)]
    pub fn ccr3_h(&mut self) -> Ccr3HW<Ccr3Spec> {
        Ccr3HW::new(self, 16)
    }
}
#[doc = "capture/compare register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr3Spec;
impl crate::RegisterSpec for Ccr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr3::R`](R) reader structure"]
impl crate::Readable for Ccr3Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr3::W`](W) writer structure"]
impl crate::Writable for Ccr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR3 to value 0"]
impl crate::Resettable for Ccr3Spec {}
