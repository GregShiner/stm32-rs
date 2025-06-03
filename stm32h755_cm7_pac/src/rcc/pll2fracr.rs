#[doc = "Register `PLL2FRACR` reader"]
pub type R = crate::R<Pll2fracrSpec>;
#[doc = "Register `PLL2FRACR` writer"]
pub type W = crate::W<Pll2fracrSpec>;
#[doc = "Field `FRACN2` reader - Fractional part of the multiplication factor for PLL VCO"]
pub type Fracn2R = crate::FieldReader<u16>;
#[doc = "Field `FRACN2` writer - Fractional part of the multiplication factor for PLL VCO"]
pub type Fracn2W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL VCO"]
    #[inline(always)]
    pub fn fracn2(&self) -> Fracn2R {
        Fracn2R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL VCO"]
    #[inline(always)]
    pub fn fracn2(&mut self) -> Fracn2W<Pll2fracrSpec> {
        Fracn2W::new(self, 3)
    }
}
#[doc = "RCC PLL2 Fractional Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll2fracr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2fracr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll2fracrSpec;
impl crate::RegisterSpec for Pll2fracrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll2fracr::R`](R) reader structure"]
impl crate::Readable for Pll2fracrSpec {}
#[doc = "`write(|w| ..)` method takes [`pll2fracr::W`](W) writer structure"]
impl crate::Writable for Pll2fracrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL2FRACR to value 0"]
impl crate::Resettable for Pll2fracrSpec {}
