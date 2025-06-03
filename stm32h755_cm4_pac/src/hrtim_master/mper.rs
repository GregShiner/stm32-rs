#[doc = "Register `MPER` reader"]
pub type R = crate::R<MperSpec>;
#[doc = "Register `MPER` writer"]
pub type W = crate::W<MperSpec>;
#[doc = "Field `MPER` reader - Master Timer Period value"]
pub type MperR = crate::FieldReader<u16>;
#[doc = "Field `MPER` writer - Master Timer Period value"]
pub type MperW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Master Timer Period value"]
    #[inline(always)]
    pub fn mper(&self) -> MperR {
        MperR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Timer Period value"]
    #[inline(always)]
    pub fn mper(&mut self) -> MperW<MperSpec> {
        MperW::new(self, 0)
    }
}
#[doc = "Master Timer Period Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MperSpec;
impl crate::RegisterSpec for MperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mper::R`](R) reader structure"]
impl crate::Readable for MperSpec {}
#[doc = "`write(|w| ..)` method takes [`mper::W`](W) writer structure"]
impl crate::Writable for MperSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MPER to value 0xffff"]
impl crate::Resettable for MperSpec {
    const RESET_VALUE: u32 = 0xffff;
}
