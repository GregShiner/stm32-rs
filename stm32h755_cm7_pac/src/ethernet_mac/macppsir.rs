#[doc = "Register `MACPPSIR` reader"]
pub type R = crate::R<MacppsirSpec>;
#[doc = "Register `MACPPSIR` writer"]
pub type W = crate::W<MacppsirSpec>;
#[doc = "Field `PPSINT0` reader - PPSINT0"]
pub type Ppsint0R = crate::FieldReader<u32>;
#[doc = "Field `PPSINT0` writer - PPSINT0"]
pub type Ppsint0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PPSINT0"]
    #[inline(always)]
    pub fn ppsint0(&self) -> Ppsint0R {
        Ppsint0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PPSINT0"]
    #[inline(always)]
    pub fn ppsint0(&mut self) -> Ppsint0W<MacppsirSpec> {
        Ppsint0W::new(self, 0)
    }
}
#[doc = "PPS interval register\n\nYou can [`read`](crate::Reg::read) this register and get [`macppsir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacppsirSpec;
impl crate::RegisterSpec for MacppsirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macppsir::R`](R) reader structure"]
impl crate::Readable for MacppsirSpec {}
#[doc = "`write(|w| ..)` method takes [`macppsir::W`](W) writer structure"]
impl crate::Writable for MacppsirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACPPSIR to value 0"]
impl crate::Resettable for MacppsirSpec {}
