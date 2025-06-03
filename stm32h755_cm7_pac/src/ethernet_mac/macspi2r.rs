#[doc = "Register `MACSPI2R` reader"]
pub type R = crate::R<Macspi2rSpec>;
#[doc = "Register `MACSPI2R` writer"]
pub type W = crate::W<Macspi2rSpec>;
#[doc = "Field `SPI2` reader - SPI2"]
pub type Spi2R = crate::FieldReader<u16>;
#[doc = "Field `SPI2` writer - SPI2"]
pub type Spi2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SPI2"]
    #[inline(always)]
    pub fn spi2(&self) -> Spi2R {
        Spi2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SPI2"]
    #[inline(always)]
    pub fn spi2(&mut self) -> Spi2W<Macspi2rSpec> {
        Spi2W::new(self, 0)
    }
}
#[doc = "PTP Source port identity 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macspi2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macspi2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Macspi2rSpec;
impl crate::RegisterSpec for Macspi2rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macspi2r::R`](R) reader structure"]
impl crate::Readable for Macspi2rSpec {}
#[doc = "`write(|w| ..)` method takes [`macspi2r::W`](W) writer structure"]
impl crate::Writable for Macspi2rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACSPI2R to value 0"]
impl crate::Resettable for Macspi2rSpec {}
