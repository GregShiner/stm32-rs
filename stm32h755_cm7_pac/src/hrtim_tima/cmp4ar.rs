#[doc = "Register `CMP4AR` reader"]
pub type R = crate::R<Cmp4arSpec>;
#[doc = "Register `CMP4AR` writer"]
pub type W = crate::W<Cmp4arSpec>;
#[doc = "Field `CMP4x` reader - Timerx Compare 4 value"]
pub type Cmp4xR = crate::FieldReader<u16>;
#[doc = "Field `CMP4x` writer - Timerx Compare 4 value"]
pub type Cmp4xW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 4 value"]
    #[inline(always)]
    pub fn cmp4x(&self) -> Cmp4xR {
        Cmp4xR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 4 value"]
    #[inline(always)]
    pub fn cmp4x(&mut self) -> Cmp4xW<Cmp4arSpec> {
        Cmp4xW::new(self, 0)
    }
}
#[doc = "Timerx Compare 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp4ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp4ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp4arSpec;
impl crate::RegisterSpec for Cmp4arSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp4ar::R`](R) reader structure"]
impl crate::Readable for Cmp4arSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp4ar::W`](W) writer structure"]
impl crate::Writable for Cmp4arSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMP4AR to value 0"]
impl crate::Resettable for Cmp4arSpec {}
