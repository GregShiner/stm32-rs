#[doc = "Register `PLL1FRACR` reader"]
pub type R = crate::R<Pll1fracrSpec>;
#[doc = "Register `PLL1FRACR` writer"]
pub type W = crate::W<Pll1fracrSpec>;
#[doc = "Field `FRACN1` reader - Fractional part of the multiplication factor for PLL1 VCO"]
pub type Fracn1R = crate::FieldReader<u16>;
#[doc = "Field `FRACN1` writer - Fractional part of the multiplication factor for PLL1 VCO"]
pub type Fracn1W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn fracn1(&self) -> Fracn1R {
        Fracn1R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn fracn1(&mut self) -> Fracn1W<Pll1fracrSpec> {
        Fracn1W::new(self, 3)
    }
}
#[doc = "RCC PLL1 Fractional Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1fracr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1fracr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll1fracrSpec;
impl crate::RegisterSpec for Pll1fracrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll1fracr::R`](R) reader structure"]
impl crate::Readable for Pll1fracrSpec {}
#[doc = "`write(|w| ..)` method takes [`pll1fracr::W`](W) writer structure"]
impl crate::Writable for Pll1fracrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL1FRACR to value 0"]
impl crate::Resettable for Pll1fracrSpec {}
