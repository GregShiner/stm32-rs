#[doc = "Register `MCMP2R` reader"]
pub type R = crate::R<Mcmp2rSpec>;
#[doc = "Register `MCMP2R` writer"]
pub type W = crate::W<Mcmp2rSpec>;
#[doc = "Field `MCMP2` reader - Master Timer Compare 2 value"]
pub type Mcmp2R = crate::FieldReader<u16>;
#[doc = "Field `MCMP2` writer - Master Timer Compare 2 value"]
pub type Mcmp2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Master Timer Compare 2 value"]
    #[inline(always)]
    pub fn mcmp2(&self) -> Mcmp2R {
        Mcmp2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Timer Compare 2 value"]
    #[inline(always)]
    pub fn mcmp2(&mut self) -> Mcmp2W<Mcmp2rSpec> {
        Mcmp2W::new(self, 0)
    }
}
#[doc = "Master Timer Compare 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcmp2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcmp2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcmp2rSpec;
impl crate::RegisterSpec for Mcmp2rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcmp2r::R`](R) reader structure"]
impl crate::Readable for Mcmp2rSpec {}
#[doc = "`write(|w| ..)` method takes [`mcmp2r::W`](W) writer structure"]
impl crate::Writable for Mcmp2rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCMP2R to value 0"]
impl crate::Resettable for Mcmp2rSpec {}
