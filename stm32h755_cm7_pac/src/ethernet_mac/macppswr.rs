#[doc = "Register `MACPPSWR` reader"]
pub type R = crate::R<MacppswrSpec>;
#[doc = "Register `MACPPSWR` writer"]
pub type W = crate::W<MacppswrSpec>;
#[doc = "Field `PPSWIDTH0` reader - PPSWIDTH0"]
pub type Ppswidth0R = crate::FieldReader<u32>;
#[doc = "Field `PPSWIDTH0` writer - PPSWIDTH0"]
pub type Ppswidth0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PPSWIDTH0"]
    #[inline(always)]
    pub fn ppswidth0(&self) -> Ppswidth0R {
        Ppswidth0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PPSWIDTH0"]
    #[inline(always)]
    pub fn ppswidth0(&mut self) -> Ppswidth0W<MacppswrSpec> {
        Ppswidth0W::new(self, 0)
    }
}
#[doc = "PPS width register\n\nYou can [`read`](crate::Reg::read) this register and get [`macppswr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppswr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacppswrSpec;
impl crate::RegisterSpec for MacppswrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macppswr::R`](R) reader structure"]
impl crate::Readable for MacppswrSpec {}
#[doc = "`write(|w| ..)` method takes [`macppswr::W`](W) writer structure"]
impl crate::Writable for MacppswrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACPPSWR to value 0"]
impl crate::Resettable for MacppswrSpec {}
