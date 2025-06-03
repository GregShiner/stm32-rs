#[doc = "Register `APB3FZ2` reader"]
pub type R = crate::R<Apb3fz2Spec>;
#[doc = "Register `APB3FZ2` writer"]
pub type W = crate::W<Apb3fz2Spec>;
#[doc = "Field `WWDG1` reader - WWDG1 stop in debug"]
pub type Wwdg1R = crate::BitReader;
#[doc = "Field `WWDG1` writer - WWDG1 stop in debug"]
pub type Wwdg1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - WWDG1 stop in debug"]
    #[inline(always)]
    pub fn wwdg1(&self) -> Wwdg1R {
        Wwdg1R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - WWDG1 stop in debug"]
    #[inline(always)]
    pub fn wwdg1(&mut self) -> Wwdg1W<Apb3fz2Spec> {
        Wwdg1W::new(self, 6)
    }
}
#[doc = "DBGMCU APB3 peripheral freeze register CPU2\n\nYou can [`read`](crate::Reg::read) this register and get [`apb3fz2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3fz2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb3fz2Spec;
impl crate::RegisterSpec for Apb3fz2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb3fz2::R`](R) reader structure"]
impl crate::Readable for Apb3fz2Spec {}
#[doc = "`write(|w| ..)` method takes [`apb3fz2::W`](W) writer structure"]
impl crate::Writable for Apb3fz2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB3FZ2 to value 0"]
impl crate::Resettable for Apb3fz2Spec {}
