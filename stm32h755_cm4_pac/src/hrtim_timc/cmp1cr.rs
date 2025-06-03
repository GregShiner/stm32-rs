#[doc = "Register `CMP1CR` reader"]
pub type R = crate::R<Cmp1crSpec>;
#[doc = "Register `CMP1CR` writer"]
pub type W = crate::W<Cmp1crSpec>;
#[doc = "Field `CMP1x` reader - Timerx Compare 1 value"]
pub type Cmp1xR = crate::FieldReader<u16>;
#[doc = "Field `CMP1x` writer - Timerx Compare 1 value"]
pub type Cmp1xW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 1 value"]
    #[inline(always)]
    pub fn cmp1x(&self) -> Cmp1xR {
        Cmp1xR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 1 value"]
    #[inline(always)]
    pub fn cmp1x(&mut self) -> Cmp1xW<Cmp1crSpec> {
        Cmp1xW::new(self, 0)
    }
}
#[doc = "Timerx Compare 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp1cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp1crSpec;
impl crate::RegisterSpec for Cmp1crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp1cr::R`](R) reader structure"]
impl crate::Readable for Cmp1crSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp1cr::W`](W) writer structure"]
impl crate::Writable for Cmp1crSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMP1CR to value 0"]
impl crate::Resettable for Cmp1crSpec {}
