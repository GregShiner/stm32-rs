#[doc = "Register `REPBR` reader"]
pub type R = crate::R<RepbrSpec>;
#[doc = "Register `REPBR` writer"]
pub type W = crate::W<RepbrSpec>;
#[doc = "Field `REPx` reader - Timerx Repetition counter value"]
pub type RepxR = crate::FieldReader;
#[doc = "Field `REPx` writer - Timerx Repetition counter value"]
pub type RepxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timerx Repetition counter value"]
    #[inline(always)]
    pub fn repx(&self) -> RepxR {
        RepxR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timerx Repetition counter value"]
    #[inline(always)]
    pub fn repx(&mut self) -> RepxW<RepbrSpec> {
        RepxW::new(self, 0)
    }
}
#[doc = "Timerx Repetition Register\n\nYou can [`read`](crate::Reg::read) this register and get [`repbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RepbrSpec;
impl crate::RegisterSpec for RepbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`repbr::R`](R) reader structure"]
impl crate::Readable for RepbrSpec {}
#[doc = "`write(|w| ..)` method takes [`repbr::W`](W) writer structure"]
impl crate::Writable for RepbrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REPBR to value 0"]
impl crate::Resettable for RepbrSpec {}
