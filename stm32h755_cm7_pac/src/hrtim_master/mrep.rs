#[doc = "Register `MREP` reader"]
pub type R = crate::R<MrepSpec>;
#[doc = "Register `MREP` writer"]
pub type W = crate::W<MrepSpec>;
#[doc = "Field `MREP` reader - Master Timer Repetition counter value"]
pub type MrepR = crate::FieldReader;
#[doc = "Field `MREP` writer - Master Timer Repetition counter value"]
pub type MrepW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Master Timer Repetition counter value"]
    #[inline(always)]
    pub fn mrep(&self) -> MrepR {
        MrepR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Master Timer Repetition counter value"]
    #[inline(always)]
    pub fn mrep(&mut self) -> MrepW<MrepSpec> {
        MrepW::new(self, 0)
    }
}
#[doc = "Master Timer Repetition Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mrep::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrep::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrepSpec;
impl crate::RegisterSpec for MrepSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrep::R`](R) reader structure"]
impl crate::Readable for MrepSpec {}
#[doc = "`write(|w| ..)` method takes [`mrep::W`](W) writer structure"]
impl crate::Writable for MrepSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MREP to value 0"]
impl crate::Resettable for MrepSpec {}
