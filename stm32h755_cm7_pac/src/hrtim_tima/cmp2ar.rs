#[doc = "Register `CMP2AR` reader"]
pub type R = crate::R<Cmp2arSpec>;
#[doc = "Register `CMP2AR` writer"]
pub type W = crate::W<Cmp2arSpec>;
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
    pub fn cmp2x(&mut self) -> Cmp2xW<Cmp2arSpec> {
        Cmp2xW::new(self, 0)
    }
}
#[doc = "Timerx Compare 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp2ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp2ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp2arSpec;
impl crate::RegisterSpec for Cmp2arSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp2ar::R`](R) reader structure"]
impl crate::Readable for Cmp2arSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp2ar::W`](W) writer structure"]
impl crate::Writable for Cmp2arSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMP2AR to value 0"]
impl crate::Resettable for Cmp2arSpec {}
