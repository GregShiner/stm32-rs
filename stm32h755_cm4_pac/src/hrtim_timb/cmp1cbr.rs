#[doc = "Register `CMP1CBR` reader"]
pub type R = crate::R<Cmp1cbrSpec>;
#[doc = "Register `CMP1CBR` writer"]
pub type W = crate::W<Cmp1cbrSpec>;
#[doc = "Field `CMP1x` reader - Timerx Compare 1 value"]
pub type Cmp1xR = crate::FieldReader<u16>;
#[doc = "Field `CMP1x` writer - Timerx Compare 1 value"]
pub type Cmp1xW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REPx` reader - Timerx Repetition value (aliased from HRTIM_REPx register)"]
pub type RepxR = crate::FieldReader;
#[doc = "Field `REPx` writer - Timerx Repetition value (aliased from HRTIM_REPx register)"]
pub type RepxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 1 value"]
    #[inline(always)]
    pub fn cmp1x(&self) -> Cmp1xR {
        Cmp1xR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Timerx Repetition value (aliased from HRTIM_REPx register)"]
    #[inline(always)]
    pub fn repx(&self) -> RepxR {
        RepxR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 1 value"]
    #[inline(always)]
    pub fn cmp1x(&mut self) -> Cmp1xW<Cmp1cbrSpec> {
        Cmp1xW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Timerx Repetition value (aliased from HRTIM_REPx register)"]
    #[inline(always)]
    pub fn repx(&mut self) -> RepxW<Cmp1cbrSpec> {
        RepxW::new(self, 16)
    }
}
#[doc = "Timerx Compare 1 Compound Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp1cbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1cbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp1cbrSpec;
impl crate::RegisterSpec for Cmp1cbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp1cbr::R`](R) reader structure"]
impl crate::Readable for Cmp1cbrSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp1cbr::W`](W) writer structure"]
impl crate::Writable for Cmp1cbrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMP1CBR to value 0"]
impl crate::Resettable for Cmp1cbrSpec {}
