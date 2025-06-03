#[doc = "Register `CMP1DR` reader"]
pub type R = crate::R<Cmp1drSpec>;
#[doc = "Register `CMP1DR` writer"]
pub type W = crate::W<Cmp1drSpec>;
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
    pub fn cmp1x(&mut self) -> Cmp1xW<Cmp1drSpec> {
        Cmp1xW::new(self, 0)
    }
}
#[doc = "Timerx Compare 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp1dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp1drSpec;
impl crate::RegisterSpec for Cmp1drSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp1dr::R`](R) reader structure"]
impl crate::Readable for Cmp1drSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp1dr::W`](W) writer structure"]
impl crate::Writable for Cmp1drSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMP1DR to value 0"]
impl crate::Resettable for Cmp1drSpec {}
