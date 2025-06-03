#[doc = "Register `PLL3FRACR` reader"]
pub type R = crate::R<Pll3fracrSpec>;
#[doc = "Register `PLL3FRACR` writer"]
pub type W = crate::W<Pll3fracrSpec>;
#[doc = "Field `FRACN3` reader - Fractional part of the multiplication factor for PLL3 VCO"]
pub type Fracn3R = crate::FieldReader<u16>;
#[doc = "Field `FRACN3` writer - Fractional part of the multiplication factor for PLL3 VCO"]
pub type Fracn3W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL3 VCO"]
    #[inline(always)]
    pub fn fracn3(&self) -> Fracn3R {
        Fracn3R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL3 VCO"]
    #[inline(always)]
    pub fn fracn3(&mut self) -> Fracn3W<Pll3fracrSpec> {
        Fracn3W::new(self, 3)
    }
}
#[doc = "RCC PLL3 Fractional Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll3fracr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3fracr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll3fracrSpec;
impl crate::RegisterSpec for Pll3fracrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll3fracr::R`](R) reader structure"]
impl crate::Readable for Pll3fracrSpec {}
#[doc = "`write(|w| ..)` method takes [`pll3fracr::W`](W) writer structure"]
impl crate::Writable for Pll3fracrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL3FRACR to value 0"]
impl crate::Resettable for Pll3fracrSpec {}
