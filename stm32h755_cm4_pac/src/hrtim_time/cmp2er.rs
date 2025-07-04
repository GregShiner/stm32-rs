#[doc = "Register `CMP2ER` reader"]
pub type R = crate::R<Cmp2erSpec>;
#[doc = "Register `CMP2ER` writer"]
pub type W = crate::W<Cmp2erSpec>;
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
    pub fn cmp2x(&mut self) -> Cmp2xW<Cmp2erSpec> {
        Cmp2xW::new(self, 0)
    }
}
#[doc = "Timerx Compare 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp2er::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp2er::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp2erSpec;
impl crate::RegisterSpec for Cmp2erSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp2er::R`](R) reader structure"]
impl crate::Readable for Cmp2erSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp2er::W`](W) writer structure"]
impl crate::Writable for Cmp2erSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMP2ER to value 0"]
impl crate::Resettable for Cmp2erSpec {}
