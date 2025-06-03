#[doc = "Register `MACSPI0R` reader"]
pub type R = crate::R<Macspi0rSpec>;
#[doc = "Register `MACSPI0R` writer"]
pub type W = crate::W<Macspi0rSpec>;
#[doc = "Field `SPI0` reader - SPI0"]
pub type Spi0R = crate::FieldReader<u32>;
#[doc = "Field `SPI0` writer - SPI0"]
pub type Spi0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SPI0"]
    #[inline(always)]
    pub fn spi0(&self) -> Spi0R {
        Spi0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SPI0"]
    #[inline(always)]
    pub fn spi0(&mut self) -> Spi0W<Macspi0rSpec> {
        Spi0W::new(self, 0)
    }
}
#[doc = "PTP Source Port Identity 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`macspi0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macspi0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Macspi0rSpec;
impl crate::RegisterSpec for Macspi0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macspi0r::R`](R) reader structure"]
impl crate::Readable for Macspi0rSpec {}
#[doc = "`write(|w| ..)` method takes [`macspi0r::W`](W) writer structure"]
impl crate::Writable for Macspi0rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACSPI0R to value 0"]
impl crate::Resettable for Macspi0rSpec {}
