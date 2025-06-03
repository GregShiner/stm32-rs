#[doc = "Register `MCMP3R` reader"]
pub type R = crate::R<Mcmp3rSpec>;
#[doc = "Register `MCMP3R` writer"]
pub type W = crate::W<Mcmp3rSpec>;
#[doc = "Field `MCMP3` reader - Master Timer Compare 3 value"]
pub type Mcmp3R = crate::FieldReader<u16>;
#[doc = "Field `MCMP3` writer - Master Timer Compare 3 value"]
pub type Mcmp3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Master Timer Compare 3 value"]
    #[inline(always)]
    pub fn mcmp3(&self) -> Mcmp3R {
        Mcmp3R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Timer Compare 3 value"]
    #[inline(always)]
    pub fn mcmp3(&mut self) -> Mcmp3W<Mcmp3rSpec> {
        Mcmp3W::new(self, 0)
    }
}
#[doc = "Master Timer Compare 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcmp3r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcmp3r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcmp3rSpec;
impl crate::RegisterSpec for Mcmp3rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcmp3r::R`](R) reader structure"]
impl crate::Readable for Mcmp3rSpec {}
#[doc = "`write(|w| ..)` method takes [`mcmp3r::W`](W) writer structure"]
impl crate::Writable for Mcmp3rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCMP3R to value 0"]
impl crate::Resettable for Mcmp3rSpec {}
