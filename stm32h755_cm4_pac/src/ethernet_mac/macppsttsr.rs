#[doc = "Register `MACPPSTTSR` reader"]
pub type R = crate::R<MacppsttsrSpec>;
#[doc = "Register `MACPPSTTSR` writer"]
pub type W = crate::W<MacppsttsrSpec>;
#[doc = "Field `TSTRH0` reader - TSTRH0"]
pub type Tstrh0R = crate::FieldReader<u32>;
#[doc = "Field `TSTRH0` writer - TSTRH0"]
pub type Tstrh0W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - TSTRH0"]
    #[inline(always)]
    pub fn tstrh0(&self) -> Tstrh0R {
        Tstrh0R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - TSTRH0"]
    #[inline(always)]
    pub fn tstrh0(&mut self) -> Tstrh0W<MacppsttsrSpec> {
        Tstrh0W::new(self, 0)
    }
}
#[doc = "PPS target time seconds register\n\nYou can [`read`](crate::Reg::read) this register and get [`macppsttsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsttsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacppsttsrSpec;
impl crate::RegisterSpec for MacppsttsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macppsttsr::R`](R) reader structure"]
impl crate::Readable for MacppsttsrSpec {}
#[doc = "`write(|w| ..)` method takes [`macppsttsr::W`](W) writer structure"]
impl crate::Writable for MacppsttsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACPPSTTSR to value 0"]
impl crate::Resettable for MacppsttsrSpec {}
