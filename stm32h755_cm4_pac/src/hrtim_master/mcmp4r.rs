#[doc = "Register `MCMP4R` reader"]
pub type R = crate::R<Mcmp4rSpec>;
#[doc = "Register `MCMP4R` writer"]
pub type W = crate::W<Mcmp4rSpec>;
#[doc = "Field `MCMP4` reader - Master Timer Compare 4 value"]
pub type Mcmp4R = crate::FieldReader<u16>;
#[doc = "Field `MCMP4` writer - Master Timer Compare 4 value"]
pub type Mcmp4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Master Timer Compare 4 value"]
    #[inline(always)]
    pub fn mcmp4(&self) -> Mcmp4R {
        Mcmp4R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Timer Compare 4 value"]
    #[inline(always)]
    pub fn mcmp4(&mut self) -> Mcmp4W<Mcmp4rSpec> {
        Mcmp4W::new(self, 0)
    }
}
#[doc = "Master Timer Compare 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcmp4r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcmp4r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcmp4rSpec;
impl crate::RegisterSpec for Mcmp4rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcmp4r::R`](R) reader structure"]
impl crate::Readable for Mcmp4rSpec {}
#[doc = "`write(|w| ..)` method takes [`mcmp4r::W`](W) writer structure"]
impl crate::Writable for Mcmp4rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCMP4R to value 0"]
impl crate::Resettable for Mcmp4rSpec {}
