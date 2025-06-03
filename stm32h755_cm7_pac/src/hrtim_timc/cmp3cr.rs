#[doc = "Register `CMP3CR` reader"]
pub type R = crate::R<Cmp3crSpec>;
#[doc = "Register `CMP3CR` writer"]
pub type W = crate::W<Cmp3crSpec>;
#[doc = "Field `CMP3x` reader - Timerx Compare 3 value"]
pub type Cmp3xR = crate::FieldReader<u16>;
#[doc = "Field `CMP3x` writer - Timerx Compare 3 value"]
pub type Cmp3xW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 3 value"]
    #[inline(always)]
    pub fn cmp3x(&self) -> Cmp3xR {
        Cmp3xR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 3 value"]
    #[inline(always)]
    pub fn cmp3x(&mut self) -> Cmp3xW<Cmp3crSpec> {
        Cmp3xW::new(self, 0)
    }
}
#[doc = "Timerx Compare 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp3cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp3cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp3crSpec;
impl crate::RegisterSpec for Cmp3crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp3cr::R`](R) reader structure"]
impl crate::Readable for Cmp3crSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp3cr::W`](W) writer structure"]
impl crate::Writable for Cmp3crSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMP3CR to value 0"]
impl crate::Resettable for Cmp3crSpec {}
