#[doc = "Register `APB3RSTR` reader"]
pub type R = crate::R<Apb3rstrSpec>;
#[doc = "Register `APB3RSTR` writer"]
pub type W = crate::W<Apb3rstrSpec>;
#[doc = "Field `LTDCRST` reader - LTDC block reset"]
pub type LtdcrstR = crate::BitReader;
#[doc = "Field `LTDCRST` writer - LTDC block reset"]
pub type LtdcrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - LTDC block reset"]
    #[inline(always)]
    pub fn ltdcrst(&self) -> LtdcrstR {
        LtdcrstR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - LTDC block reset"]
    #[inline(always)]
    pub fn ltdcrst(&mut self) -> LtdcrstW<Apb3rstrSpec> {
        LtdcrstW::new(self, 3)
    }
}
#[doc = "RCC APB3 Peripheral Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb3rstrSpec;
impl crate::RegisterSpec for Apb3rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb3rstr::R`](R) reader structure"]
impl crate::Readable for Apb3rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb3rstr::W`](W) writer structure"]
impl crate::Writable for Apb3rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB3RSTR to value 0"]
impl crate::Resettable for Apb3rstrSpec {}
