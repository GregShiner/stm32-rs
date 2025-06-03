#[doc = "Register `CMP1ER` reader"]
pub type R = crate::R<Cmp1erSpec>;
#[doc = "Register `CMP1ER` writer"]
pub type W = crate::W<Cmp1erSpec>;
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
    pub fn cmp1x(&mut self) -> Cmp1xW<Cmp1erSpec> {
        Cmp1xW::new(self, 0)
    }
}
#[doc = "Timerx Compare 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp1er::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1er::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp1erSpec;
impl crate::RegisterSpec for Cmp1erSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp1er::R`](R) reader structure"]
impl crate::Readable for Cmp1erSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp1er::W`](W) writer structure"]
impl crate::Writable for Cmp1erSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMP1ER to value 0"]
impl crate::Resettable for Cmp1erSpec {}
