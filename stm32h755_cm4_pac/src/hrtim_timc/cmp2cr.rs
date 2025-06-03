#[doc = "Register `CMP2CR` reader"]
pub type R = crate::R<Cmp2crSpec>;
#[doc = "Register `CMP2CR` writer"]
pub type W = crate::W<Cmp2crSpec>;
#[doc = "Field `CMP2x` reader - Timerx Compare 2 value"]
pub type Cmp2xR = crate::FieldReader<u16>;
#[doc = "Field `CMP2x` writer - Timerx Compare 2 value"]
pub type Cmp2xW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 2 value"]
    #[inline(always)]
    pub fn cmp2x(&self) -> Cmp2xR {
        Cmp2xR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 2 value"]
    #[inline(always)]
    pub fn cmp2x(&mut self) -> Cmp2xW<Cmp2crSpec> {
        Cmp2xW::new(self, 0)
    }
}
#[doc = "Timerx Compare 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp2crSpec;
impl crate::RegisterSpec for Cmp2crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp2cr::R`](R) reader structure"]
impl crate::Readable for Cmp2crSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp2cr::W`](W) writer structure"]
impl crate::Writable for Cmp2crSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMP2CR to value 0"]
impl crate::Resettable for Cmp2crSpec {}
