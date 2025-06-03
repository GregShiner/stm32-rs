#[doc = "Register `MACSPI1R` reader"]
pub type R = crate::R<Macspi1rSpec>;
#[doc = "Register `MACSPI1R` writer"]
pub type W = crate::W<Macspi1rSpec>;
#[doc = "Field `SPI1` reader - SPI1"]
pub type Spi1R = crate::FieldReader<u32>;
#[doc = "Field `SPI1` writer - SPI1"]
pub type Spi1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SPI1"]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SPI1"]
    #[inline(always)]
    pub fn spi1(&mut self) -> Spi1W<Macspi1rSpec> {
        Spi1W::new(self, 0)
    }
}
#[doc = "PTP Source port identity 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macspi1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macspi1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Macspi1rSpec;
impl crate::RegisterSpec for Macspi1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macspi1r::R`](R) reader structure"]
impl crate::Readable for Macspi1rSpec {}
#[doc = "`write(|w| ..)` method takes [`macspi1r::W`](W) writer structure"]
impl crate::Writable for Macspi1rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACSPI1R to value 0"]
impl crate::Resettable for Macspi1rSpec {}
